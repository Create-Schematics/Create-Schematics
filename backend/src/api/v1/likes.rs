use poem_openapi::OpenApi;
use poem_openapi::param::{Path, Query};
use poem_openapi_derive::Enum;
use poem::web::Data;
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::error::ApiError;
use crate::response::ApiResult;
use crate::api::ApiContext;

pub (in crate::api::v1) struct LikesApi;

#[derive(Deserialize, Debug, Enum)]
pub (in crate::api::v1) enum LikeAction {
    #[serde(rename = "like")]
    Like,

    #[serde(rename = "dislike")]
    Dislike
}

impl LikeAction {
    #[inline]
    pub fn positive(self) -> bool { 
        matches!(self, Self::Like)
    }
}

#[OpenApi(prefix_path="/api/v1")]
impl LikesApi {
    
    /// Adds either a like or dislike reaction to a schematic by the current user.
    /// If this user has already liked or disliked this schematic, their reaction
    /// will be updated instead.
    /// 
    /// If you are looking to remove a like see the `DELETE /api/v1/schematics/:id/like`
    ///  
    #[oai(path = "/schematics/:id/like", method="get")]
    async fn like_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,    
        Session(user_id): Session,
        Path(schematic_id): Path<Uuid>,
        Query(query): Query<LikeAction>
    ) -> ApiResult<()> {
        sqlx::query!(
            r#"
            insert into schematic_likes (
                schematic_id, user_id,
                positive
            )
            values (
                $1, $2, $3
            )
            on conflict (
                schematic_id, user_id
            )
            do update set positive = $3
            "#,
            schematic_id,
            user_id,
            query.positive()
        )
        .execute(&ctx.pool)
        .await?;
    
        Ok(())
    }
    
    /// Removes a like or dislike reaction from a schematic by the current user.
    /// 
    /// If the user hasnt already liked the schematic or a schematic with the given
    /// id doesnt exist then a `404 Not Found` error will be returned
    /// 
    #[oai(path = "/schematics/:id/like", method="delete")]
    async fn remove_like_from_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,    
        Session(user_id): Session,
        Path(schematic_id): Path<Uuid>,
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;
    
        let result = sqlx::query!(
            r#"
            with deleted_like as (
                delete from schematic_likes
                where user_id = $1
                and schematic_id = $2
                returning 1
            )
            select
                exists(select 1 from deleted_like) "deleted"
            "#,
            user_id,
            schematic_id
        )
        .fetch_one(&mut *transaction)
        .await?;
    
        transaction.commit().await?;
    
        if result.deleted.unwrap_or_default() {
            Ok(())
        } else {
            Err(ApiError::NotFound)
        }
    }
}

