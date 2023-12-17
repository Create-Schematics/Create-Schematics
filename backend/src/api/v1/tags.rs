use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::{Query, Path};
use poem_openapi::payload::Json;
use poem_openapi_derive::Object;
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::error::ApiError;
use crate::response::ApiResult;
use crate::api::ApiContext;


pub (in crate::api::v1) struct TagsApi;

#[derive(Debug, Deserialize, Object)]
pub (in crate::api::v1) struct Tags {
    pub tag_names: Vec<i64>,
}

#[derive(Debug, Serialize, Object)]
pub (in crate::api::v1) struct FullTag {
    pub tag_id: i64,
    pub tag_name: String,
}

#[OpenApi(prefix_path="/api/v1")]
impl TagsApi {

    /// Fetch all the tags applied to a given schematic
    /// 
    /// This also includes the name of each tag aswell as their underlying id
    /// 
    #[oai(path = "/schematics/:id/tags", method = "get")]
    async fn get_schematic_tags(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(schematic_id): Path<Uuid>
    ) -> ApiResult<Json<Vec<FullTag>>> {
        let tags = sqlx::query_as!(
            FullTag,
            r#"
            select tag_id, tag_name
            from applied_tags
            inner join tags using (tag_id)
            where schematic_id = $1 
            "#,
            schematic_id
        )
        .fetch_all(&ctx.pool)
        .await?;

        Ok(Json(tags))
    }

    /// Fetch a number of the valid tags available within the api aswell as their
    /// given names. If no limit is specified 20 will be returned by default.
    /// 
    /// If you are looking to get all of the tags on a specific schematic see
    /// `GET /api/v1/schematics/{id}/tags`
    /// 
    #[oai(path = "/tags", method = "get")]
    async fn get_valid_tags(
        &self,
        Data(ctx): Data<&ApiContext>,
        Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>
    ) -> ApiResult<Json<Vec<FullTag>>> {
        let tags = sqlx::query_as!(
            FullTag,
            r#"
            select tag_id, tag_name
            from tags
            limit $1 offset $2
            "#,
            limit.unwrap_or(20),
            offset.unwrap_or(0)
        )
        .fetch_all(&ctx.pool)
        .await?;

        Ok(Json(tags))
    }

    /// Applies tags to a given schematic given their identifiers see
    /// `GET /api/v1/tags` for a full list of valid tags
    /// 
    /// This requires for the current user to be the schematics author
    /// 
    #[oai(path = "/schematis/:id/tags", method = "post")]
    async fn tag_schematic_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Session(user_id): Session,
        Path(schematic_id): Path<Uuid>,
        Json(query): Json<Tags>
    ) -> ApiResult<()> {
        let schematic_meta = sqlx::query!(
            r#"select author from schematics where schematic_id = $1"#,
            schematic_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)?;

        if schematic_meta.author != user_id {
            return Err(ApiError::Forbidden.into());
        }

        sqlx::query!(
            // Unfortunately sqlx does not inserting multiple records 
            // directly without using a query builder which would mean 
            // loosing out on compiler checking. None of this is ideal
            // if you have a better solution please let us know.
            // 
            // see: https://github.com/launchbadge/sqlx/issues/294
            r#"
            insert into applied_tags (
                schematic_id, tag_id
            )
            select $1, tag_id
            from unnest($2::bigint[]) as tag_id
            on conflict do nothing
            "#,
            schematic_id,
            &query.tag_names[..],
        )
        .execute(&ctx.pool)
        .await?;

        Ok(())
    }

    /// Removes tags from a given schematic given their identifiers
    /// 
    /// This requires for the current user to be the schematics author
    /// 
    #[oai(path = "/schematis/:id/tags", method = "delete")]
    async fn untag_schematic_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Session(user_id): Session,
        Path(schematic_id): Path<Uuid>,
        Json(query): Json<Tags>
    ) -> ApiResult<()> {
        let schematic_meta = sqlx::query!(
            r#"select author from schematics where schematic_id = $1"#,
            schematic_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)?;

        if schematic_meta.author != user_id {
            return Err(ApiError::Forbidden.into());
        }

        sqlx::query!(
            r#"
            delete from applied_tags 
            where schematic_id = $1 
            and tag_id = ANY($2)
            "#,
            schematic_id,
            &query.tag_names
        )
        .execute(&ctx.pool)
        .await?;

        Ok(())
    }
}