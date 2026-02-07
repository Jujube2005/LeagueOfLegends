use anyhow::{Ok, Result};
use async_trait::async_trait;
use chrono::{Duration, Utc};
// use diesel::{
//     ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, insert_into,
//     query_dsl::methods::{FilterDsl, SelectDsl},
// };
use diesel::{dsl::insert_into, prelude::*};
use std::sync::Arc;

use crate::{
    config::config_loader::get_jwt_env,
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        repositories::BrawlerRepository,
        value_objects::{
            base64_img::Base64Img, brawler_model::BrawlerModel, mission_model::MissionModel,
            mission_summary::MissionSummaryModel, uploaded_img::UploadedImg,
        },
    },
    infrastructure::{
        cloudinary::{self, UploadImageOptions},
        database::{
            postgresql_connection::PgPoolSquad,
            schema::{brawlers, crew_memberships},
        },
        jwt::{
            generate_token,
            jwt_model::{Claims, Passport},
        },
    },
};

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<Passport> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let user_id = insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        let display_name = register_brawler_entity.display_name;

        let jwt_env = get_jwt_env()?;
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (Utc::now() + Duration::days(jwt_env.ttl)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        let token = generate_token(jwt_env.secret, &claims)?;
        Ok(Passport {
            token,
            display_name,
            avatar_url: None,
        })
    }

    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_id(&self, id: i32) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .find(id)
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_email(&self, email: String) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::email.eq(email))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn upload_base64img(
        &self,
        user_id: i32,
        base64img: Base64Img,
        opt: UploadImageOptions,
    ) -> Result<UploadedImg> {
        let uploaded_img = cloudinary::upload(base64img, opt).await?;

        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(user_id))
            .set((
                brawlers::avatar_url.eq(uploaded_img.url.clone()),
                brawlers::avatar_public_id.eq(uploaded_img.public_id.clone()),
            ))
            .execute(&mut conn)?;

        Ok(uploaded_img)
    }

    // *เพิ่ม
    async fn get_leaderboard(&self) -> Result<Vec<BrawlerModel>> {
        use diesel::sql_query;
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = sql_query(
            "SELECT id, display_name, COALESCE(avatar_url, '') as avatar_url, mission_success_count, mission_join_count 
             FROM brawlers 
             ORDER BY mission_success_count DESC, mission_join_count DESC, id ASC 
             LIMIT 10",
        )
        .load::<BrawlerModel>(&mut conn)?;

        Ok(result)
    }

    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        // Use a raw SQL query to select the MissionModel fields including
        // the chief's display name and the crew count.
        let sql = r#"
SELECT
    missions.id,
    missions.name,
    missions.description,
    missions.category,
    missions.max_crew,
    missions.status,
    missions.chief_id,
    brawlers.display_name AS chief_display_name,
    (SELECT COUNT(*) FROM crew_memberships WHERE crew_memberships.mission_id = missions.id) AS crew_count,
    false AS is_member,
    missions.created_at,
    missions.updated_at
FROM missions
LEFT JOIN brawlers ON brawlers.id = missions.chief_id
WHERE missions.deleted_at IS NULL
    AND missions.chief_id = $1
ORDER BY missions.created_at DESC
        "#;

        let results = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(brawler_id)
            .load::<MissionModel>(&mut conn)?;

        Ok(results)
    }

    async fn get_mission_summary(&self, brawler_id: i32) -> Result<MissionSummaryModel> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let sql = r#"
SELECT
    (SELECT COUNT(*)::BIGINT FROM missions WHERE chief_id = $1 AND deleted_at IS NULL) AS created_count,
    (SELECT COUNT(*)::BIGINT FROM crew_memberships WHERE brawler_id = $1) AS joined_count,
    (
        (SELECT COUNT(*)::BIGINT
        FROM missions m
        INNER JOIN crew_memberships cm ON m.id = cm.mission_id
        WHERE cm.brawler_id = $1 AND m.status = 'Completed')
        +
        (SELECT COUNT(*)::BIGINT
        FROM missions m2
        WHERE m2.chief_id = $1 AND m2.status = 'Completed')
    ) AS completed_count,
    (
        (SELECT COUNT(*)::BIGINT
        FROM missions m
        INNER JOIN crew_memberships cm ON m.id = cm.mission_id
        WHERE cm.brawler_id = $1 AND m.status = 'Failed')
        +
        (SELECT COUNT(*)::BIGINT
        FROM missions m3
        WHERE m3.chief_id = $1 AND m3.status = 'Failed')
    ) AS failed_count
        "#;

        let summary = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(brawler_id)
            .get_result::<MissionSummaryModel>(&mut conn)?;

        Ok(summary)
    }

    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .first::<i64>(&mut conn)?;

        let count = u32::try_from(result)?;

        Ok(count)
    }

    async fn get_all_brawlers(&self) -> Result<Vec<BrawlerModel>> {
        use diesel::sql_query;
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = sql_query(
            "SELECT id, display_name, COALESCE(avatar_url, '') as avatar_url, mission_success_count, mission_join_count 
             FROM brawlers 
             ORDER BY display_name ASC",
        )
        .load::<BrawlerModel>(&mut conn)?;

        Ok(result)
    }
}
