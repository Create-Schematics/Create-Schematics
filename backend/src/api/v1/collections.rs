use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi_derive::{OpenApi, Object, Multipart};
use uuid::Uuid;

use crate::authentication::session::{Session, OptionalSession};
use crate::error::ApiError;
use crate::middleware::validators::Profanity;
use crate::response::ApiResult;
use crate::api::ApiContext;

pub (in crate::api::v1) struct CollectionsApi;

#[derive(Serialize, Debug, Object)]
pub (in crate::api::v1) struct Collection {
    pub collection_id: Uuid,
    pub collection_name: String,
    pub user_id: Uuid,
    pub is_private: bool,
}

#[derive(Serialize, Debug, Object)]
pub (in crate::api::v1) struct UserCollection {
    pub collection_id: Uuid,
    pub collection_name: String,
    pub is_private: bool,
    pub entries: Vec<Uuid>,
}

#[derive(Serialize, Debug, Object)]
pub (in crate::api::v1) struct FullCollection {
    pub collection_id: Uuid,
    pub collection_name: String,
    pub is_private: bool,
    pub user_id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub entries: Vec<Uuid>,
}

#[derive(Multipart, Debug)]
pub (in crate::api::v1) struct UpdateCollection {
    pub is_private: Option<bool>,
    #[oai(validator(min_length=3, max_length=50, custom="Profanity"))]
    pub collection_name: Option<String>,
}

#[derive(Multipart, Debug)]
pub (in crate::api::v1) struct CollectionBuilder {
    pub is_private: bool,
    #[oai(validator(min_length=3, max_length=50, custom="Profanity"))]
    pub collection_name: String,
}

#[derive(Multipart, Object, Debug)]
pub (in crate::api::v1) struct CollectionEntry {
    pub schematic_id: Uuid,
}

#[OpenApi(prefix_path="/v1")]
impl CollectionsApi {

    /// Fetches a number of collections that contain a given schematic including
    /// the schematic ids of there entries and basic information about their
    /// author such as their username and avatar to avoid subsequent requests.
    /// 
    /// Note that private collections even if the user requesting them is the
    /// owner will not be returned from this endpoint.
    /// 
    #[oai(path = "/schematics/:schematic_id/collections", method = "get")]
    async fn collections_containing_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(schematic_id): Path<Uuid>,
        Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>
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
                left join collection_entries using (collection_id)
            where
                $1 = schematic_id
                and is_private = false
            group by
                collection_id,
                avatar,
                username
            limit $2 offset $3
            "#,
            schematic_id,
            limit.unwrap_or(20),
            offset.unwrap_or(0)
        )
        .fetch_all(&ctx.pool)
        .await?;
    
        Ok(Json(collections))
    }
    
    /// Fetches a collection by it's id asell as the ids of all the schematics
    /// it contains and some information about the author such as their username
    /// and avatar url.
    /// 
    /// If the requested collection is private and the user is not it's owner
    /// then `404 Not Found` will be returned even if the collection does exist
    /// for privacy
    /// 
    #[oai(path = "/collections/:collection_id", method = "get")]
    async fn get_collection_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(collection_id): Path<Uuid>,
        OptionalSession(user_id): OptionalSession
    ) -> ApiResult<Json<FullCollection>> {
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
                left join collection_entries using (collection_id)
            where
                $1 = collection_id
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

    /// Fetches all public collections owned by a given user, this will include
    /// additional information about each collection such as it's entries but
    /// will not include information about the author 
    /// 
    /// If you need to get all collections including private ones from a user
    /// refer to `/api/v1/collections` which fetches collections owned by the 
    /// current user
    /// 
    #[oai(path = "/users/:user_id/collections", method = "get")]
    async fn get_users_collections(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(user_id): Path<Uuid>,
        Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>
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
                left join collection_entries using (collection_id)
            where
                $1 = user_id
                and is_private = false
            group by
                collection_id
            limit $2 offset $3
            "#,
            user_id,
            limit.unwrap_or(20),
            offset.unwrap_or(0)
        )
        .fetch_all(&ctx.pool)
        .await?;
    
        Ok(Json(schematics))
    }

    /// Fetches all collections, including private ones owned by the current
    /// user, this will include all of a collections entries but will not 
    /// include information about the owner of the user as it is assumed this
    /// information is already known. 
    /// 
    /// If you need to get collections from another user refer to 
    /// `GET /api/v1/users/{id}/collections`, this returns all the collections
    /// that are public and owned by a given user
    /// 
    #[oai(path = "/collections", method = "get")]
    async fn get_current_users_collections(
        &self,
        Data(ctx): Data<&ApiContext>,
        Session(user_id): Session,
        Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>
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
                left join collection_entries using (collection_id)
            where
                $1 = user_id
                and is_private = false
            group by
                collection_id
            limit $2 offset $3
            "#,
            user_id,
            limit.unwrap_or(20),
            offset.unwrap_or(0)
        )
        .fetch_all(&ctx.pool)
        .await?;

        Ok(Json(schematics))
    }

    /// Creates a new collection for the current user with a given name and
    /// privacy level, new collections will always be empty, aswell as this
    /// it is assumed information about the current user is already known and
    /// so will not be returned by the api.
    /// 
    #[oai(path = "/collections", method = "post")]
    async fn create_new_collection(
        &self,
        Data(ctx): Data<&ApiContext>,
        Session(user_id): Session,
        form: CollectionBuilder
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
            user_id,
        )
        .fetch_one(&mut *transaction)
        .await?;
    
        transaction.commit().await?;
    
        Ok(Json(collection))
    }

    /// Updatess a given collection, all fields are optional but at least one is
    /// required as well as this the current user must either own the collection
    /// or have permissions to mdoerate posts to edit the collection. 
    /// 
    /// If you are looking to add a schematic to add a schematic to the collection
    /// see `POST /api/v1/collections/{id}/schematics`
    /// 
    #[oai(path = "/collections/:collection_id", method = "patch")]
    async fn update_collection_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(collection_id): Path<Uuid>,
        Session(user_id): Session,
        form: UpdateCollection
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        let collection_meta = sqlx::query!(
            r#"select user_id from collections where collection_id = $1"#,
            collection_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;
        
        if collection_meta.user_id != user_id {
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

    /// Fetches the ids of all the schematics in a collection. If the given
    /// collection is private then the current user must be it's owner. If the 
    /// user is now the owner of the collection they will recieve a 
    /// `404 Not Found`, even if the given collection was found, for privacy.
    /// 
    /// If you are looking to fetch information about the collection itself
    /// see `GET /collections/:id`
    /// 
    #[oai(path = "/collections/:collection_id/schematics", method = "get")]
    async fn fetch_collection_entries(
        &self,
        Data(ctx): Data<&ApiContext>,
        OptionalSession(user_id): OptionalSession,
        Path(collection_id): Path<Uuid>,
    ) -> ApiResult<Json<Vec<CollectionEntry>>> {

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
            // Mask the existance of the collection
            return Err(ApiError::NotFound);
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

    /// Adds a schematic to a collection, the current user must own the given
    /// collection in order to add to it. The same schematic cannot be added to
    /// a given colleciton twice, if the collection already contains the new
    /// schematic then a `409 Conflict` will be returned.
    /// 
    #[oai(path = "/collections/:collection_id/schematics", method = "post")]
    async fn add_schematic_to_collection(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(collection_id): Path<Uuid>,
        Session(user_id): Session,
        form: CollectionEntry
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;
        
        let collection_meta = sqlx::query!(
            r#"select user_id from collections where collection_id = $1"#,
            collection_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;
        
        if collection_meta.user_id != user_id {
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
            form.schematic_id,
            collection_id,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    /// Removes a given schematic from a colleciton, this requires the current
    /// user to be the collections owner. 
    /// 
    /// If you are looking to entirely remove a collection not just specific 
    /// schematics within it see `DELETE /api/v1/collections/:id`
    /// 
    #[oai(path = "/collections/:collection_id/schematics", method = "delete")]
    async fn remove_schematic_from_collection(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(collection_id): Path<Uuid>,
        Session(user_id): Session,
        form: CollectionEntry
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;
    
        let collection_meta = sqlx::query!(
            r#"select user_id from collections where collection_id = $1"#,
            collection_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;
    
        if collection_meta.user_id != user_id {
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

    /// Removes a collection entirely aswell as all attached entries. This 
    /// requires for the current user to either own the collection or have 
    /// permissions to moderate posts.
    /// 
    /// If you are looking to remove a specific schematic from a collection
    /// see `DELETE /api/v1/collections/:id/schematics`
    /// 
    #[oai(path = "/collections/:collection_id", method = "delete")]
    async fn remove_collection_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(collection_id): Path<Uuid>,
        session: Session,
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        let collection_meta = sqlx::query!(
            r#"select user_id from collections where collection_id = $1"#,
            collection_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;
        
        if collection_meta.user_id != session.user_id() &&
                !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Unauthorized);
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
}