use std::time::Duration;

use poem::web::Data;
use poem_openapi::payload::Json;
use poem_openapi::param::Path;
use poem_openapi_derive::{OpenApi, Object, Multipart};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::api::ApiContext;
use crate::authentication::schemes::{Session, TIMEOUT_NAMESPACE};
use crate::error::{ApiError, Punishment};
use crate::response::ApiResult;

pub (in crate::api::v1) struct ModerationApi;

#[derive(Serialize, Object, Debug)]
pub (in crate::api::v1) struct Timeout {
    pub punishment_id: Uuid,
    pub user_id: Uuid,
    pub issuer_id: Uuid,
    pub reason: Option<String>,
    pub until: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime
}

#[derive(Deserialize, Multipart, Debug)]
pub (in crate::api::v1) struct TimeoutBuilder {
    pub duration: Option<u64>,
    pub reason: Option<String>
}

#[OpenApi(prefix_path="/v1")]
impl ModerationApi {
    #[oai(path="/users/:username/timeout", method="get")]
    async fn fetch_user_timeouts(
        &self,
        Data(ctx): Data<&ApiContext>,  
        Path(username): Path<String>,
    ) -> ApiResult<Json<Vec<Timeout>>> {
        let punishments = sqlx::query_as!(
            Timeout,
            r#"
            select
                punishment_id, issuer_id,
                user_id, reason, until, 
                created_at
            from
                punishments
            where
                user_id = (select user_id from users where username = $1)
            "#,
            username
        )
        .fetch_all(&ctx.pool)
        .await?;

        Ok(Json(punishments))
    }

    #[oai(path="/users/:username/timeout", method="put")]
    async fn timeout_user(
        &self,
        Data(ctx): Data<&ApiContext>,  
        Path(username): Path<String>,
        session: Session,
        form: TimeoutBuilder
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        if !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Forbidden);
        }

        let until = form.duration
            .map(|seconds| Duration::from_secs(seconds))
            .map(|duration| OffsetDateTime::now_utc() + duration);

        let user_id = session.user_id();

        sqlx::query!(
            r#"
            insert into punishments (
                user_id, reason,
                issuer_id, until
            )
            values (
                (select user_id from users where username = $1), 
                $2, $3, $4
            )
            "#,
            username,
            form.reason,
            user_id,
            until
        )
        .execute(&mut *transaction)
        .await?;

        let punishment = Punishment {
            until,
            reason: form.reason
        };

        ctx.redis_pool
            .set_json(TIMEOUT_NAMESPACE, user_id, punishment, form.duration)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[oai(path="/timeout/:punishment_id", method="get")]
    async fn fetch_timeout_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,  
        Path(punishment_id): Path<Uuid>
    ) -> ApiResult<Json<Timeout>> {
        sqlx::query_as!(
            Timeout,
            r#"
            select
                punishment_id, issuer_id,
                user_id, reason, until, 
                created_at
            from
                punishments
            where
                punishment_id = $1
            "#,
            punishment_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
    }

    #[oai(path="/timeout/:punishment_id", method="delete")]
    async fn clear_punishment(
        &self,
        Data(ctx): Data<&ApiContext>,  
        Path(punishment_id): Path<Uuid>,
        session: Session
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        if !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Forbidden);
        }

        sqlx::query!(
            r#"
            delete from punishments
            where punishment_id = $1
            "#,
            punishment_id
        )
        .execute(&mut *transaction)
        .await?;

        ctx.redis_pool.delete(TIMEOUT_NAMESPACE, punishment_id).await?;
        transaction.commit().await?;

        Ok(())
    }
}