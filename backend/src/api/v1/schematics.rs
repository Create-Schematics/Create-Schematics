use core::fmt;

use axum::routing::get;
use axum::{Router, Json};
use axum::extract::{State, Path, Query};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::error::{ApiError, ResultExt};
use crate::models::user::{User, Permissions};
use crate::response::ApiResult;
use crate::models::schematic::Schematic;
use crate::api::ApiContext;

#[derive(Debug, Serialize, ToSchema)]
pub (in crate::api) struct FullSchematic {
    #[schema(min_length=16, max_length=16)]
    pub schematic_id: String,

    #[schema(example="My schematic")]
    pub schematic_name: String,

    pub author: Uuid,

    #[schema(example="Rabbitminers")]
    #[schema(min_length=3, max_length=20)]
    pub author_name: String,

    #[schema(example=0)]
    pub favorite_count: i64,

    #[schema(example=0)]
    pub like_count: i64,

    #[schema(example=0)]
    pub dislike_count: i64,

    #[schema(example=0)]
    pub downloads: i64,

    pub tags: Vec<i64>,

    #[schema(example=4, minimum=1)]
    pub game_version_id: i64,

    #[schema(example="1.18.2")]
    pub game_version_name: String,

    #[schema(example=8, minimum=1)]
    pub create_version_id: i64,

    #[schema(example="0.5.1")]
    pub create_version_name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct SchematicBuilder {
    /// The name of the new schematic
    /// 
    #[schema(min_length=3, max_length=50)]
    pub schematic_name: String,
    
    /// The id of the game version of the new schematic
    /// 
    #[schema(example=4, minimum=1)]
    pub game_version: i32,
    
    /// The id of the create version of the new schematic
    /// 
    #[schema(example=8, minimum=1)]
    pub create_version: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct UpdateSchematic {
    /// The new name for the schematic
    ///
    #[schema(min_length=3, max_length=50)]
    pub schematic_name: Option<String>,
    
    /// The id of the new game version of the schematic
    /// 
    #[schema(example=4, minimum=1)]
    pub game_version: Option<i32>,
    
    /// The id of the new create version of the schematic
    /// 
    #[schema(example=8, minimum=1)]
    pub create_version: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) struct SearchQuery {
    /// The maximum number of schematics to fetch. If this is not 
    /// provided it will default to 20. No more than 50 schematics
    /// can be fetched at once. 
    ///
    #[schema(example=20, minimum=0, maximum=50)]
    pub limit: Option<i64>,
    
    /// The page of schematics to fetch from. If this is not provided
    /// it will default to page 0 (no offset).
    /// 
    #[schema(example=0, minimum=0)]
    pub offset: Option<i64>,

    /// The tags to fetch from, only schematics with all of these tags
    /// will fetched.
    /// 
    pub tag_ids: Option<Vec<i64>>,
    
    /// The term to search schematics for. Both schematic names and
    /// descriptions will be matched agaisnt the this term.
    /// 
    #[schema(example="test")]
    pub term: String,

    /// The order in which schematics similar to the query should
    /// be ordered. By default the ones created most recently will
    /// be shown first.
    /// 
    #[schema(example="likes")]
    pub sort: Option<SortBy>
}

#[derive(Debug, Deserialize, ToSchema)]
pub (in crate::api) enum SortBy {
    /// Fetch the schematics with the most downloads first
    /// 
    #[serde(rename = "downloads")]
    Downloads,

    /// Fetch the schematics with the most likes first. This does not
    /// account for the number of dislikes
    /// 
    #[serde(rename = "likes")]
    Likes,

    /// Fetch the most recently created schematics first.
    /// 
    #[serde(rename = "created-at")]
    CreatedAt
}

impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortBy::Downloads => write!(f, "downloads"),
            SortBy::Likes => write!(f, "likes"),
            SortBy::CreatedAt => write!(f, "created_at")
        }
    }
}

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route("/schematics", 
            get(search_schematics)
            .post(upload_schematic)
        )
        .route(
            "/schematics/:id",
            get(get_schematic_by_id)
            .patch(update_schematic_by_id)
            .delete(delete_schematic_by_id)
        )
}

#[utoipa::path(
    get,
    path = "/schematics/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("id" = String, Path, description = "The id of the schematic to fetch")
    ),
    responses(
        (status = 200, description = "Successfully retrieved the schematic", body = Schematic, content_type = "application/json"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn get_schematic_by_id(
    State(ctx): State<ApiContext>,
    Path(schematic_id): Path<Uuid>
) -> ApiResult<Json<FullSchematic>> {
    // This needs some testing, overall we have two options for
    // selecting the number of favourites, likes and dislikes. 
    // We can either join the respective tables then filter them
    // as done in this query or select them in a sub-query
    //
    // coalesce((
    //    select count(*) from schematic_likes likes 
    //    where likes.schematic_id = schematics.schematic_id 
    //    and positive = false
    // ), 0) as "dislike_count!"
    // 
    // It's not clear which would actually perform better so some
    // testing would be useful. The other other option we have is
    // to just store a count in the table then update that
    //
    sqlx::query_as!(
        FullSchematic,
        r#"
        select 
            schematic_id, 
            schematic_name, 
            author, 
            username as author_name, 
            downloads,
            create_version_id, 
            create_version_name,
            game_version_id, 
            game_version_name, 
            coalesce(array_agg(tag_id) filter (where tag_id is not null), array []::bigint[]) as "tags!",
            coalesce(count(likes.schematic_id) filter (where positive = true), 0) as "like_count!",
            coalesce(count(likes.schematic_id) filter (where positive = false), 0) as "dislike_count!",
            coalesce(count(favorites.schematic_id), 0) as "favorite_count!"
        from 
            schematics
            inner join create_versions using (create_version_id)
            inner join game_versions using (game_version_id)
            inner join users on user_id = author
            left join schematic_likes likes using (schematic_id)
            left join favorites using (schematic_id)
            left join applied_tags using (schematic_id)
        where 
            schematic_id = $1
        group by 
            schematic_id,
            game_version_id,
            game_version_name,
            username,
            create_version_name
        "#,
        schematic_id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::NotFound)
    .map(Json)
}

#[utoipa::path(
    patch,
    path = "/schematics/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = String, Path, description = "The id of the schematic to update")
    ),
    request_body(
        content = UpdateSchematic, description = "The values to update", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully updated the schematic", body = Schematic, content_type = "application/json"),
        (status = 401, description = "You need to be logged in to update a schematic"),
        (status = 403, description = "You do not have permission to update this schematic"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 422, description = "A schematic with the new name already exists"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn update_schematic_by_id(
    State(ctx): State<ApiContext>,
    Path(schematic_id): Path<Uuid>,
    user: User,
    Json(schematic): Json<UpdateSchematic>,
) -> ApiResult<Json<Schematic>> {
    let mut transaction = ctx.pool.begin().await?;

    Schematic::check_user_permissions(user, &schematic_id, Permissions::MODERATE_POSTS, &mut *transaction).await?;

    let schematic = sqlx::query_as!(
        Schematic,
        r#"
        update schematics
            set
                schematic_name = coalesce($1, schematic_name),
                game_version_id = coalesce($2, game_version_id),
                create_version_id = coalesce($3, create_version_id)
            where schematic_id = $4
            returning
                schematic_id,
                schematic_name,
                game_version_id,
                create_version_id,
                author,
                downloads
        "#,
        schematic.schematic_name,
        schematic.game_version,
        schematic.create_version,
        schematic_id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)?;

    transaction.commit().await?;

    Ok(Json(schematic))
}

#[utoipa::path(
    delete,
    path = "/schematics/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("schematic_id" = String, Path, description = "The id of the schematic to remove")
    ),
    responses(
        (status = 200, description = "Successfully deleted the schematic"),
        (status = 401, description = "You need to be logged in to delete a schematic"),
        (status = 403, description = "You do not have permission to delete this schematic"),
        (status = 404, description = "A schematic with that id was not found"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(("session_cookie" = []))
)]
async fn delete_schematic_by_id(
    State(ctx): State<ApiContext>,
    Path(schematic_id): Path<Uuid>,
    user: User
) -> ApiResult<()> {
    let mut transaction = ctx.pool.begin().await?;

    // This permission check could potentially be nicely merged into the
    // subsequent query by checking like so in the where clause.
    //
    // and (author = $2 or (select permissions from users where user_id = $2) & $3 = $3)
    Schematic::check_user_permissions(user, &schematic_id, Permissions::MODERATE_POSTS, &mut *transaction).await?;

    // We dont need to ensure the user owns the schematic here or that they are the owner
    // as that has already been checked and in doing so validated that the schematic exists
    sqlx::query!(
        r#"
        delete from schematics
        where schematic_id = $1
        "#,
        schematic_id,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}

#[utoipa::path(
    post,
    path = "/schematics",
    context_path = "/api/v1",
    tag = "v1",
    request_body(
        content = SchematicBuilder, description = "Information about the new schematic", content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successfully uploaded the schematic", body = Schematic, content_type = "application/json"),
        (status = 401, description = "You must be logged in to upload a schematic"),
        (status = 403, description = "You do not have permission to upload a schematic"),
        (status = 500, description = "An error occurred while uploading the schematic")
    ),
    security(("session_cookie" = []))
)]
async fn upload_schematic(
    State(ctx): State<ApiContext>,
    session: Session,
    Json(schematic): Json<SchematicBuilder>,
) -> ApiResult<Json<Schematic>> {
    let mut transaction = ctx.pool.begin().await?;

    let schematic = sqlx::query_as!(
        Schematic,
        r#"
        insert into schematics (
            schematic_name, author,
            game_version_id, 
            create_version_id
        )
        values (
            $1, $2, $3, $4
        )
        returning
            schematic_id,
            schematic_name,
            game_version_id,
            create_version_id,
            author,
            downloads
        "#,
        schematic.schematic_name,
        session.user_id,
        schematic.game_version,
        schematic.create_version
    )
    .fetch_one(&mut *transaction)
    .await
    .on_constraint("schematics_game_version_id_fkey", |_| {
        ApiError::unprocessable_entity([("game_version", "that version does not exist")])
    })
    .on_constraint("schematics_create_version_id_fkey", |_| {
        ApiError::unprocessable_entity([("create_version", "that version does not exist")])
    })?;

    transaction.commit().await?;

    Ok(Json(schematic))
}


#[utoipa::path(
    get,
    path = "/schematics",
    context_path = "/api/v1",
    tag = "v1",
    params(
        ("query" = SearchQuery, Query, description = "The number and offset of schematics to fetch")
    ),
    responses(
        (status = 200, description = "Successfully retrieved the schematics", body = [FullSchematic], content_type = "application/json"),
        (status = 500, description = "An internal server error occurred")
    ),
    security(())
)]
async fn search_schematics(
    State(ctx): State<ApiContext>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<Json<Vec<FullSchematic>>> {
    let tags = query.tag_ids.unwrap_or_default();
    let ordering = query.sort.unwrap_or(SortBy::CreatedAt);

    let schematics = sqlx::query_as!(
        FullSchematic,
        r#"
        select 
            schematic_id, 
            schematic_name, 
            author, 
            username as author_name, 
            downloads,
            create_version_id, 
            create_version_name,
            game_version_id,
            game_version_name,
            coalesce(array_agg(tag_id) filter (where tag_id is not null), array []::bigint[]) as "tags!",
            coalesce(count(likes.schematic_id) filter (where positive = true), 0) as "like_count!",
            coalesce(count(likes.schematic_id) filter (where positive = false), 0) as "dislike_count!",
            coalesce(count(favorites.schematic_id), 0) as "favorite_count!"
        from 
            schematics
            inner join create_versions using (create_version_id)
            inner join game_versions using (game_version_id)
            inner join users on user_id = author
            left join schematic_likes likes using (schematic_id)
            left join favorites using (schematic_id)
            left join applied_tags using (schematic_id)
        where 
            schematic_name % $1
            and (array_length($2::bigint[], 1) is null or tag_id = any($2))
        group by 
            schematic_id,
            game_version_id,
            game_version_name,
            username,
            create_version_id,
            create_version_name
        order by $3
        limit $4 offset $5
        "#,
        query.term,
        &tags,
        ordering.to_string(),
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0)
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(schematics))
}