use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::{OpenApi, Object};
use poem_openapi_derive::Multipart;
use uuid::Uuid;

use crate::authentication::session::Session;
use crate::error::ApiError;
use crate::{response::ApiResult, api::ApiContext};

pub (in crate::api::v1) struct ModApi;

#[derive(Serialize, Object, Debug)]
pub (in crate::api::v1) struct Mod {
    pub mod_id: Uuid,
    pub mod_slug: String,
    pub mod_name: Option<String>,
    pub curseforge_slug: Option<i32>,
    pub modrinth_slug: Option<String>
}

#[derive(Deserialize, Multipart, Debug)]
pub (in crate::api::v1) struct UpdateMod {
    pub mod_slug: Option<String>,
    pub mod_name: Option<String>,
    pub curseforge_slug: Option<i32>,
    pub modrinth_slug: Option<String>
}

#[derive(Serialize, Object, Debug)]
pub (in crate::api::v1) struct ModProposal {
    pub proposal_id: Uuid,
    pub user_id: Uuid,
    pub mod_id: Uuid,
    pub mod_name: Option<String>,
    pub mod_slug: Option<String>,
    pub curseforge_slug: Option<i32>,
    pub modrinth_slug: Option<String>
}

#[OpenApi(prefix_path="/v1")]
impl ModApi {
    #[oai(path = "/mods/:mod_id", method = "get")]
    async fn fetch_mod_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,        
        Path(mod_id): Path<Uuid>
    ) -> ApiResult<Json<Mod>> {
        sqlx::query_as!(
            Mod,
            r#"
            select
                mod_id, curseforge_slug, 
                mod_name, modrinth_slug,
                mod_slug
            from 
                mods
            where
                mod_id = $1
            "#,
            mod_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
    }

    #[oai(path = "/mods/:mod_id", method = "patch")]
    async fn update_mod_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,        
        Path(mod_id): Path<Uuid>,
        form: UpdateMod,
        session: Session
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;
        let user = session.user(&mut *transaction).await?;

        if user.is_moderator() {
            sqlx::query!(
                r#"
                update mods
                    set
                        mod_slug = coalesce($1, mod_slug),
                        mod_name = coalesce($2, mod_name),
                        curseforge_slug = coalesce($3, curseforge_slug),
                        modrinth_slug = coalesce($4, modrinth_slug)
                    where
                        mod_id = $5
                "#,
                form.mod_slug,
                form.mod_name,
                form.curseforge_slug,
                form.modrinth_slug,
                mod_id
            )
            .execute(&mut *transaction)
            .await?;
        } else {
            // If the user doesnt have sufficient permissions we create a proposal
            // that can then be later approved by a moderator or other user with
            // sufficient permissions see `PUT /mods/proposals/:proposal_id/approve` 
            // and `PUT /mods/proposals/:proposal_id/deny`
            sqlx::query!(
                r#"
                insert into mod_proposals (
                    mod_id, user_id, mod_slug, mod_name,
                    curseforge_slug, modrinth_slug
                )
                values (
                    $1, $2, $3, $4, $5, $6
                )
                "#,
                mod_id,
                user.user_id,
                form.mod_slug,
                form.mod_name,
                form.curseforge_slug,
                form.modrinth_slug
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;

        Ok(())
    }

    #[oai(path = "/mods/proposals", method = "get")]
    async fn fetch_proposals(
        &self,
        Data(ctx): Data<&ApiContext>,     
        #[oai(validator(maximum(value="50")))] Query(limit): Query<Option<i64>>,
        Query(offset): Query<Option<i64>>,
        session: Session
    ) -> ApiResult<Json<Vec<ModProposal>>> {
        if !session.is_moderator(&ctx.pool).await? {
            return Err(ApiError::Forbidden);
        }

        let proposals = sqlx::query_as!(
            ModProposal,
            r#"
            select 
                proposal_id, user_id, mod_id,
                mod_slug, mod_name, curseforge_slug,
                modrinth_slug
            from
                mod_proposals
            order by 
                created_at desc
            limit $1 offset $2
            "#,
            limit.unwrap_or(20),
            offset.unwrap_or(0)
        )
        .fetch_all(&ctx.pool)
        .await?;

        Ok(Json(proposals))
    }

    #[oai(path = "/mods/proposals/:proposal_id", method = "get")]
    async fn fetch_proposal_by_id(
        &self,
        Data(ctx): Data<&ApiContext>,    
        Path(proposal_id): Path<Uuid>, 
        session: Session
    ) -> ApiResult<Json<ModProposal>> {
        if !session.is_moderator(&ctx.pool).await? {
            return Err(ApiError::Forbidden);
        }

        sqlx::query_as!(
            ModProposal,
            r#"
            select 
                proposal_id, user_id, mod_id,
                mod_slug, mod_name, curseforge_slug,
                modrinth_slug
            from
                mod_proposals
            where
                proposal_id = $1
            "#,
            proposal_id
        )
        .fetch_optional(&ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
    }

    #[oai(path = "/mods/proposals/:proposal_id/approve", method = "put")]
    async fn approve_proposal(
        &self,
        Data(ctx): Data<&ApiContext>,     
        Path(proposal_id): Path<Uuid>,
        session: Session
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        if !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Forbidden);
        }

        let proposal = sqlx::query_as!(
            ModProposal,
            r#"
            select 
                proposal_id, user_id, mod_id,
                mod_slug, mod_name, curseforge_slug,
                modrinth_slug
            from
                mod_proposals
            where
                proposal_id = $1
            "#,
            proposal_id
        )
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or(ApiError::NotFound)?;

        // This could be done in one query by selecting each field inline but due to
        // the number of fields its likely that this would be slower than fetching 
        // everything at once in a prior query as seen above. Especially since this
        // is designed assuming the database is hosted on the same machine
        sqlx::query!(
            r#"
            update mods
                set
                    mod_slug = coalesce($1, mod_slug),
                    mod_name = coalesce($2, mod_name),
                    curseforge_slug = coalesce($3, curseforge_slug),
                    modrinth_slug = coalesce($4, modrinth_slug)
                where
                    mod_id = $5
            "#,
            proposal.mod_slug,
            proposal.mod_name,
            proposal.curseforge_slug,
            proposal.modrinth_slug,
            proposal.mod_id
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    #[oai(path = "/mods/proposals/:proposal_id/reject", method = "put")]
    async fn deny_proposal(
        &self,
        Data(ctx): Data<&ApiContext>,     
        Path(proposal_id): Path<Uuid>,
        session: Session
    ) -> ApiResult<()> {
        let mut transaction = ctx.pool.begin().await?;

        if !session.is_moderator(&mut *transaction).await? {
            return Err(ApiError::Forbidden);
        }

        sqlx::query!(
            r#"
            delete from mod_proposals
            where proposal_id = $1
            "#,
            proposal_id
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}