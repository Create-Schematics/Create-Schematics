use std::time::Duration;

use poem::web::Data;
use poem_openapi::payload::Json;
use poem_openapi::param::{Path, Query};
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
    pub username: String,
    pub displayname: Option<String>,
    pub issuer_id: Uuid,
    pub issuer_username: String,
    pub issuer_displayname: Option<String>,
    pub reason: Option<String>,
    pub until: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime
}

#[derive(Deserialize, Multipart, Debug)]
pub (in crate::api::v1) struct TimeoutBuilder {
    pub duration: Option<u64>,
    pub reason: Option<String>
}

#[derive(Serialize, Object, Debug)]
pub (in crate::api::v1) struct Report {
    pub report_id: Uuid,
    pub user_id: Uuid,
    pub schematic_id: Uuid,
    pub body: Option<String>,
    pub created_at: OffsetDateTime
}

#[derive(Serialize, Object, Debug)]
pub (in crate::api::v1) struct FullReport {
    pub report_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub displayname: Option<String>,
    pub schematic_id: Uuid,
    pub schematic_name: String,
    pub body: Option<String>,
    pub created_at: OffsetDateTime
}

#[derive(Deserialize, Multipart, Debug)]
pub (in crate::api::v1) struct ReportBuilder {
    pub schematic_id: Uuid,
    pub body: Option<String>
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
                punishment_id, 
                reason, 
                until, 
                target.user_id as user_id, 
                target.username as username,
                target.displayname as displayname,
                issuer_id,
                issuer.username as issuer_username,
                issuer.displayname as issuer_displayname,
                punishments.created_at
            from
                punishments
                inner join users target on target.user_id = punishments.user_id
                inner join users issuer on issuer.user_id = punishments.issuer_id 
            where
                punishments.user_id = (select user_id from users where username = $1)
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
                punishment_id, 
                reason, 
                until, 
                target.user_id as user_id, 
                target.username as username,
                target.displayname as displayname,
                issuer_id,
                issuer.username as issuer_username,
                issuer.displayname as issuer_displayname,
                punishments.created_at
            from
                punishments
                inner join users target on target.user_id = punishments.user_id
                inner join users issuer on issuer.user_id = punishments.issuer_id 
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
    
    #[oai(path="/reports", method="get")]
    async fn fetch_reports(
        &self,
        Data(ctx): Data<&ApiContext>,     
        #[oai(validator(maximum(value="50")))] Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>,
        session: Session
    ) -> ApiResult<Json<Vec<FullReport>>> {
        if !session.is_moderator(&ctx.pool).await? {
            return Err(ApiError::Forbidden);
        }

        let reports = sqlx::query_as!(
            FullReport,
            r#"
            select
                report_id,
                reports.user_id,
                username,
                displayname,
                reports.schematic_id,
                schematic_name,
                reports.body,
                reports.created_at
            from
                reports
                inner join users on reports.user_id = users.user_id
                inner join schematics on reports.schematic_id = schematics.schematic_id
            limit $1 offset $2
            "#,
            limit,
            offset
        )
        .fetch_all(&ctx.pool)
        .await?;

        Ok(Json(reports))
    }

    #[oai(path="/reports", method="post")]
    async fn report_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,     
        session: Session,
        form: ReportBuilder
    ) -> ApiResult<Json<Report>> {
        let mut transaction = ctx.pool.begin().await?;

        let report = sqlx::query_as!(
            Report,
            r#"
            insert into reports (
                user_id, schematic_id,
                body
            )
            values (
                $1, $2, $3
            )
            returning
                report_id,
                user_id,
                schematic_id,
                body,
                created_at
            "#,
            session.user_id(),
            form.schematic_id,
            form.body
        )
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(Json(report))
    }

    #[oai(path="/reports/:report_id", method = "get")]
    async fn fetch_report_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,     
        Path(report_id): Path<Uuid>,
        session: Session
    ) -> ApiResult<Json<FullReport>> {
        if !session.is_moderator(&ctx.pool).await? {
            return Err(ApiError::Forbidden);
        }

        sqlx::query_as!(
            FullReport,
            r#"
            select
                report_id,
                reports.user_id,
                username,
                displayname,
                reports.schematic_id,
                schematic_name,
                reports.body,
                reports.created_at
            from
                reports
                inner join users on reports.user_id = users.user_id
                inner join schematics on reports.schematic_id = schematics.schematic_id
            where
                report_id = $1
            "#,
            report_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
    }

    #[oai(path="/reports/created", method = "get")]
    async fn fetch_current_users_reports(
        &self,
        Data(ctx): Data<&ApiContext>,     
        #[oai(validator(maximum(value="50")))] Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>,
        Session(user_id): Session
    ) -> ApiResult<Json<Vec<FullReport>>> {
        let reports = sqlx::query_as!(
            FullReport,
            r#"
            select
                report_id,
                reports.user_id,
                username,
                displayname,
                reports.schematic_id,
                schematic_name,
                reports.body,
                reports.created_at
            from
                reports
                inner join users on reports.user_id = users.user_id
                inner join schematics on reports.schematic_id = schematics.schematic_id
            where
                reports.user_id = $1
            limit $2 offset $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(&ctx.pool)
        .await?;

        Ok(Json(reports))
    }

    #[oai(path="/reports/:report_id/approve", method = "put")]
    async fn approve_report(
        &self,
        Data(ctx): Data<&ApiContext>,     
        Path(report_id): Path<Uuid>,
        session: Session
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        if !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Forbidden);
        }

        sqlx::query!(
            r#"
            delete from schematics
            where schematic_id = (select schematic_id from reports where report_id = $1)
            "#,
            report_id
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[oai(path="/reports/:report_id/reject", method = "put")]
    async fn reject_report(
        &self,
        Data(ctx): Data<&ApiContext>,     
        Path(report_id): Path<Uuid>,
        session: Session
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        if !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Forbidden);
        }

        sqlx::query!(
            r#"
            delete from reports
            where report_id = $1
            "#,
            report_id
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}