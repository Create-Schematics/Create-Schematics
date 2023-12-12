use axum::{Router, extract::{State, Path}, Json, routing::get};
use axum_typed_multipart::{TypedMultipart, TryFromMultipart};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{api::ApiContext, response::ApiResult, error::ApiError, models::user::{User, Permissions}, authentication::session::Session};

pub (in crate::api::v1) fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/schematics/:id/collections",
            get(collections_containing_schematic)
        )
        .route(
            "/users/:id/collections",
            get(get_users_collections)
        )
        .route(
            "/collections",
            get(get_current_users_collections)
            .post(create_new_collection)
        )
        .route(
            "/collections/:id",
            get(get_collection_by_id)
            .patch(update_collection_by_id)
            .delete(remove_collection_by_id)
        )
        .route(
            "/collections/:id/schematics",
            get(fetch_collection_entries)
            .post(add_schematic_to_collection)
            .delete(remove_schematic_from_collection)
        )
}

#[derive(Serialize, Debug, ToSchema)]
pub (in crate::api) struct Collection {
    pub collection_id: Uuid,
    pub collection_name: String,
    pub user_id: Uuid,
    pub is_private: bool,
}

#[derive(Serialize, Debug, ToSchema)]
pub (in crate::api) struct UserCollection {
    pub collection_id: Uuid,
    pub collection_name: String,
    pub is_private: bool,
    pub entries: Vec<Uuid>,
}

#[derive(Serialize, Debug, ToSchema)]
pub (in crate::api) struct FullCollection {
    pub collection_id: Uuid,
    pub collection_name: String,
    pub is_private: bool,
    pub user_id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub entries: Vec<Uuid>,
}

#[derive(TryFromMultipart, Debug, ToSchema)]
pub (in crate::api) struct UpdateCollection {
    pub collection_name: Option<String>,
    pub is_private: Option<bool>,
}

#[derive(TryFromMultipart, Debug, ToSchema)]
pub (in crate::api) struct CollectionBuilder {
    pub collection_name: String,
    pub is_private: bool,
}

#[derive(TryFromMultipart, Serialize, Debug, ToSchema)]
pub (in crate::api) struct CollectionEntry {
    pub schematic_id: Uuid,
}

async fn collections_containing_schematic(
    State(ctx): State<ApiContext>,
    Path(schematic_id): Path<Uuid>,
) -> ApiResult<Json<Vec<FullCollection>>> {
    let collections = sqlx::query_as!(
        FullCollection,
        r#"
        select
            collection_id, is_private,
            collection_name, user_id,
            avatar, username,
            coalesce(array_agg(schematic_id) filter (where schematic_id is not null), array []::uuid[]) as "entries!"
        from
            collections
            inner join users using (user_id)
            inner join collection_entries using (collection_id)
        where
            $1 = schematic_id
            and is_private = false
        group by
            collection_id,
            avatar,
            username
        "#,
        schematic_id
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(collections))
}

async fn get_collection_by_id(
    State(ctx): State<ApiContext>,
    Path(collection_id): Path<Uuid>,
    session: Option<Session>
) -> ApiResult<Json<FullCollection>> {
    let user_id = session.map(|s| s.user_id);

    sqlx::query_as!(
        FullCollection,
        r#"
        select
            collection_id, is_private,
            collection_name, user_id,
            avatar, username,
            coalesce(array_agg(schematic_id) filter (where schematic_id is not null), array []::uuid[]) as "entries!"
        from
            collections
            inner join users using (user_id)
            inner join collection_entries using (collection_id)
        where
            $1 = schematic_id
            and (is_private = false or user_id = $2)
        group by
            collection_id,
            avatar,
            username
        "#,
        collection_id,
        user_id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::NotFound)
    .map(Json)
}

async fn get_users_collections(
    State(ctx): State<ApiContext>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<Vec<UserCollection>>> {
    let schematics = sqlx::query_as!(
        UserCollection,
        r#"
        select
            collection_id, is_private,
            collection_name,
            coalesce(array_agg(schematic_id) filter (where schematic_id is not null), array []::uuid[]) as "entries!"
        from
            collections
            inner join collection_entries using (collection_id)
        where
            $1 = user_id
            and is_private = false
        group by
            collection_id
        "#,
        user_id
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(schematics))
}

async fn get_current_users_collections(
    State(ctx): State<ApiContext>,
    session: Session
) -> ApiResult<Json<Vec<UserCollection>>> {
    let schematics = sqlx::query_as!(
        UserCollection,
        r#"
        select
            collection_id, is_private,
            collection_name,
            coalesce(array_agg(schematic_id) filter (where schematic_id is not null), array []::uuid[]) as "entries!"
        from
            collections
            inner join collection_entries using (collection_id)
        where
            $1 = user_id
            and is_private = false
        group by
            collection_id
        "#,
        session.user_id
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(schematics))
}

async fn create_new_collection(
    State(ctx): State<ApiContext>,
    session: Session,
    TypedMultipart(form): TypedMultipart<CollectionBuilder>
) -> ApiResult<Json<Collection>> {
    let mut transaction = ctx.pool.begin().await?;

    let collection = sqlx::query_as!(
        Collection,
        r#"
        insert into collections (
            collection_name, is_private, user_id
        )
        values (
            $1, $2, $3
        )
        returning 
            collection_id,
            collection_name,
            user_id,
            is_private
        "#,
        form.collection_name,
        form.is_private,
        session.user_id,
    )
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(Json(collection))
}

async fn add_schematic_to_collection(
    State(ctx): State<ApiContext>,
    Path(collection_id): Path<Uuid>,
    session: Session,
    TypedMultipart(form): TypedMultipart<CollectionEntry>,
) -> ApiResult<()> {
    let mut transaction = ctx.pool.begin().await?;
    
    let collection_meta = sqlx::query!(
        r#"select user_id from collections where collection_id = $1"#,
        collection_id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)?;
    
    if collection_meta.user_id != session.user_id {
        return Err(ApiError::Forbidden);
    }

    sqlx::query!(
        r#"
        insert into collection_entries (
            schematic_id, collection_id
        ) 
        values (
            $1, $2
        )
        "#,
        collection_id,
        form.schematic_id,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}

async fn update_collection_by_id(
    State(ctx): State<ApiContext>,
    Path(collection_id): Path<Uuid>,
    user: User,
    TypedMultipart(form): TypedMultipart<UpdateCollection>,
) -> ApiResult<()> {
    let mut transaction = ctx.pool.begin().await?;

    let collection_meta = sqlx::query!(
        r#"select user_id from collections where collection_id = $1"#,
        collection_id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)?;
    
    if !user.permissions.contains(Permissions::MODERATE_POSTS) 
            && collection_meta.user_id != user.user_id {
        return Err(ApiError::Forbidden);
    }

    sqlx::query!(
        r#"
        update collections
            set
                collection_name = coalesce($1, collection_name),
                is_private = coalesce($2, is_private)
            where
                collection_id = $3
        "#,
        form.collection_name,
        form.is_private,
        collection_id,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}

async fn fetch_collection_entries(
    State(ctx): State<ApiContext>,
    session: Option<Session>,
    Path(collection_id): Path<Uuid>,
) -> ApiResult<Json<Vec<CollectionEntry>>> {
    let user_id = session.map(|s| s.user_id);

    // Needs testing but it should be quicker to perform an initial query
    // checking permission here rather than joining the collections table
    // and validating permissions in the fetch query.
    let collection_meta = sqlx::query!(
        r#"select user_id, is_private from collections where collection_id = $1"#,
        collection_id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    if collection_meta.is_private && user_id != Some(collection_meta.user_id) {
        return Err(ApiError::Forbidden);
    }

    let entries = sqlx::query_as!(
        CollectionEntry,
        r#"
        select schematic_id 
        from collection_entries 
        where collection_id = $1
        "#,
        collection_id,
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(entries))
}

async fn remove_schematic_from_collection(
    State(ctx): State<ApiContext>,
    Path(collection_id): Path<Uuid>,
    session: Session,
    TypedMultipart(form): TypedMultipart<CollectionEntry>,
) -> ApiResult<()> {
    let mut transaction = ctx.pool.begin().await?;

    let collection_meta = sqlx::query!(
        r#"select user_id from collections where collection_id = $1"#,
        collection_id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)?;

    if collection_meta.user_id != session.user_id {
        return Err(ApiError::Forbidden);
    }

    sqlx::query!(
        r#"
        delete from collection_entries
        where collection_id = $1
        and schematic_id = $2
        "#,
        collection_id,
        form.schematic_id,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}

async fn remove_collection_by_id(
    State(ctx): State<ApiContext>,
    Path(collection_id): Path<Uuid>,
    user: User,
) -> ApiResult<()> {
    let mut transaction = ctx.pool.begin().await?;

    let collection_meta = sqlx::query!(
        r#"select user_id from collections where collection_id = $1"#,
        collection_id
    )
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)?;
    
    if !user.permissions.contains(Permissions::MODERATE_POSTS) 
            && collection_meta.user_id != user.user_id {
        return Err(ApiError::Forbidden);
    }

    sqlx::query!(
        r#"delete from collections where collection_id = $1"#,
        collection_id
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}