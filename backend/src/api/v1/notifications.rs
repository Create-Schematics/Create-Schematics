use poem::web::Data;
use poem_openapi::{payload::Json, param::{Query, Path}};
use poem_openapi_derive::{OpenApi, Object};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{response::ApiResult, authentication::schemes::Session, api::ApiContext, error::ApiError};

pub (in crate::api::v1) struct NotificationApi;

// We don't return the id of ther user here since
// only the user the notification is sent to will
// call an endpoint reutrning this model hence we
// can assume that the information is already known
#[derive(Serialize, Object, Debug)]
pub (in crate::api::v1) struct Notification {
    pub notification_id: Uuid,
    pub title: String,
    pub body: String,
    pub link: Option<String>,
    pub created_at: OffsetDateTime
}

#[OpenApi]
impl NotificationApi {
    #[oai(path="/notifications", method="get")]
    async fn get_notifications(
        &self,
        Data(ctx): Data<&ApiContext>,  
        #[oai(validator(maximum(value="50")))] Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>,
        Session(user_id): Session
    ) -> ApiResult<Json<Vec<Notification>>> {
        let notifications = sqlx::query_as!(
            Notification,
            r#"
            select
                notification_id,
                title, body, link,
                created_at
            from
                notifications
            where
                user_id = $1
            limit $2 offset $3
            "#,
            user_id,
            limit.unwrap_or(20),
            offset.unwrap_or(0)
        )
        .fetch_all(&ctx.pool)
        .await?;

        Ok(Json(notifications))
    }

    #[oai(path="/notifications/:notification_id", method="get")]
    async fn get_notification_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(notification_id): Path<Uuid>,
        Session(user_id): Session
    ) -> ApiResult<Json<Notification>> {
        sqlx::query_as!(
            Notification,
            r#"
            select
                notification_id,
                title, body, 
                link, created_at
            from
                notifications
            where
                notification_id = $1
                and user_id = $2
            "#,
            notification_id,
            user_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
    }

    #[oai(path="/notifications/:notification_id", method="delete")]
    async fn clear_notifications_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(notification_id): Path<Uuid>,
        Session(user_id): Session
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        let result = sqlx::query!(
            r#"
            delete from notifications
            where notification_id = $1
            and user_id = $2
            "#,
            notification_id,
            user_id
        )
        .execute(&mut *transaction)
        .await?
        .rows_affected();

        transaction.commit().await?;

        if result == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }

    #[oai(path="/notifications", method="delete")]
    async fn clear_all_notifications(
        &self,
        Data(ctx): Data<&ApiContext>,
        Session(user_id): Session
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        sqlx::query!(
            r#"
            delete from notifications
            where user_id = $1
            "#,
            user_id
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}