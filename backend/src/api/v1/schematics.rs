use core::fmt;

use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi_derive::{Object, Multipart, Enum};
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::error::{ApiError, ResultExt};
use crate::middleware::files::FileUpload;
use crate::models::user::{User, Permissions};
use crate::response::ApiResult;
use crate::models::schematic::Schematic;
use crate::api::ApiContext;
use crate::storage::upload::save_schematic_files;
use crate::storage::upload;

pub (in crate::api::v1) struct SchematicsApi;

#[derive(Debug, Serialize, Object)]
pub (in crate::api::v1) struct FullSchematic {
    pub schematic_id: String,
    pub schematic_name: String,
    pub body: String,
    pub author: Uuid,
    pub author_name: String,
    pub author_avatar: Option<String>,
    pub like_count: i64,
    pub dislike_count: i64,
    pub downloads: i64,
    pub tags: Vec<i64>,
    pub images: Vec<String>,
    pub files: Vec<String>,
    pub game_version_id: i64,
    pub game_version_name: String,
    pub create_version_id: i64,
    pub create_version_name: String,
}

#[derive(Multipart, Debug)]
pub (in crate::api::v1) struct SchematicBuilder {
    #[oai(validator(min_length=3, max_length=50))]
    pub schematic_name: String,
    #[oai(validator(max_length=2048))]
    pub schematic_body: String,
    #[oai(validator(minimum(value = "1")))]
    pub game_version: i32,
    #[oai(validator(minimum(value = "1")))]
    pub create_version: i32,
    pub files: Vec<FileUpload>,
    pub images: Vec<FileUpload>,
}

#[derive(Multipart, Debug)]
pub (in crate::api::v1) struct UpdateSchematic {
    #[oai(validator(min_length=3, max_length=50))]
    pub schematic_name: Option<String>,
    #[oai(validator(minimum(value = "1")))]
    pub game_version: Option<i32>,
    #[oai(validator(minimum(value = "1")))]
    pub create_version: Option<i32>,
}

#[derive(Multipart, Debug)]
pub (in crate::api::v1) struct SearchQuery {
    #[oai(validator(minimum(value = "1"), maximum(value = "50")))]
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub tag_ids: Option<Vec<i64>>,
    pub term: Option<String>,
    pub sort: Option<SortBy>
}

#[derive(Enum, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub (in crate::api) enum SortBy {
    /// Fetch the schematics with the most downloads first
    /// 
    Downloads,

    /// Fetch the schematics with the most likes first. This does not
    /// account for the number of dislikes
    ///
    Likes,

    /// Fetch the most recently created schematics first.
    ///  
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

#[OpenApi(prefix_path="/api/v1")]
impl SchematicsApi {

    /// Fetches a given schematic by it's id including some additional information
    /// about it and it's author including like and dislike count, applied tags and
    /// the authors username and avatar in order to reduce the need for subsequent
    /// requests
    /// 
    /// If you are looking to search for schematics not for a specific one see
    /// `GET /api/v1/schematics`
    /// 
    #[oai(path = "/schematics/:id", method = "get")]
    async fn get_schematic_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
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
                body,
                author, 
                avatar as author_avatar,
                username as author_name,
                downloads,
                files,
                images,
                create_version_id, 
                create_version_name,
                game_version_id, 
                game_version_name, 
                coalesce(array_agg(tag_id) filter (where tag_id is not null), array []::bigint[]) as "tags!",
                coalesce(count(likes.schematic_id) filter (where positive = true), 0) as "like_count!",
                coalesce(count(likes.schematic_id) filter (where positive = false), 0) as "dislike_count!"
            from 
                schematics
                inner join create_versions using (create_version_id)
                inner join game_versions using (game_version_id)
                inner join users on user_id = author
                left join schematic_likes likes using (schematic_id)
                left join applied_tags using (schematic_id)
            where 
                schematic_id = $1
            group by 
                schematic_id,
                avatar,
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

    /// Updates a given schematic by it's id. All fields are optional but at
    /// least one is required
    /// 
    /// If you are looking to add or remove images from a schematic see the
    /// `/api/v1/schematics/:id/images` endpoint and for schematic files see
    /// the `/api/v1/schematics/:id/files` endpoint
    /// 
    /// This endpoint requires for the current user to either own the schematic
    /// or to have permission to moderate posts 
    /// 
    #[oai(path = "/schematics/:id", method = "post")]
    async fn update_schematic_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
        Path(schematic_id): Path<Uuid>,
        user: User,
        schematic: UpdateSchematic
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
                    body,
                    game_version_id,
                    create_version_id,
                    files,
                    images,
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

    /// Removes an existing schematic by it's id as well as all attached data
    /// such as it's files, applied tags, likes and dislikes and entries within
    /// collections
    /// 
    /// This endpoint requires for the current user to either own the schematic
    /// or to have permissiosn to moderate posts
    /// 
    #[oai(path = "/schematics/:id", method = "delete")]
    async fn delete_schematic_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,
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

        todo!("Remove schematic files");

        transaction.commit().await?;

        Ok(())
    }

    /// Searches schematics returning a given number based on requested filters
    /// with some additional information such as the like and dislike count, tags
    /// present on a schematic and the authors username and avatar in order to
    /// reduce the need for subsequent requests
    /// 
    /// If tags are included in the query then only schematics with one or more of
    /// the selected tags will be searched for. 
    /// 
    /// If no limit is specified for the number of schematics to return it will
    /// default to 20
    /// 
    #[oai(path = "/schematics", method = "get")]
    async fn search_schematics(
        &self,
        Data(ctx): Data<&ApiContext>,
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
                body,
                avatar as author_avatar,
                username as author_name, 
                downloads,
                files,
                images,
                create_version_id, 
                create_version_name,
                game_version_id,
                game_version_name,
                coalesce(array_agg(tag_id) filter (where tag_id is not null), array []::bigint[]) as "tags!",
                coalesce(count(likes.schematic_id) filter (where positive = true), 0) as "like_count!",
                coalesce(count(likes.schematic_id) filter (where positive = false), 0) as "dislike_count!"
            from 
                schematics
                inner join create_versions using (create_version_id)
                inner join game_versions using (game_version_id)
                inner join users on user_id = author
                left join schematic_likes likes using (schematic_id)
                left join applied_tags using (schematic_id)
            where 
                ($1::text is null or schematic_name % $1)
                and (array_length($2::bigint[], 1) is null or tag_id = any($2))
            group by 
                schematic_id,
                game_version_id,
                game_version_name,
                avatar,
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

    /// Uploads a new schematic for the current user 
    /// 
    /// Schematics must have at least one image and file if not the request will
    /// be rejected with `400 Bad Request`. The file names will be preserved but
    /// will be sanitized
    /// 
    /// If an invalid game version or create version is specfied a `422 Unprocessable
    /// Entity` error will be returned with a message describing the issue.
    /// 
    #[oai(path = "/schematics", method = "post")]
    async fn upload_schematic(
        &self,
        Data(ctx): Data<&ApiContext>,
        session: Session,
        form: SchematicBuilder
    ) -> ApiResult<Json<Schematic>> {
        let mut transaction = ctx.pool.begin().await?;

        let schematic_id = Uuid::new_v4();
        
        let upload_dir = upload::build_upload_directory(&schematic_id)?;
        let (files, images) = save_schematic_files(&upload_dir, form.files, form.images).await?;

        let schematic = sqlx::query_as!(
            Schematic,
            r#"
            insert into schematics (
                schematic_id, schematic_name, 
                body, author, images, files,
                game_version_id, create_version_id
            )
            values (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            returning
                schematic_id,
                schematic_name,
                body,
                game_version_id,
                create_version_id,
                images,
                files,
                author,
                downloads
            "#,
            schematic_id,
            form.schematic_name,
            form.schematic_body,
            session.user_id,
            &images,
            &files,
            form.game_version,
            form.create_version
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
        let _persist = upload_dir.into_path();

        Ok(Json(schematic))
    }
}