use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::{
    domain::{
        repositories::mission_viewing::MissionViewingRepository,
        value_objects::{
            brawler_model::BrawlerModel,
            mission_filter::MissionFilter,
            MissionModel,
        },
    },
    infrastructure::database::postgresql_connection::PgPoolSquad,
};

pub struct MissionViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionViewingRepository for MissionViewingPostgres {

    async fn crew_counting(&self, mission_id: i32) -> Result<i64> {
        use crate::infrastructure::database::schema::crew_memberships;

        let db_pool = Arc::clone(&self.db_pool);
        let count = tokio::task::spawn_blocking(move || -> Result<i64> {
            let mut conn = db_pool.get()?;
            let count = crew_memberships::table
                .filter(crew_memberships::mission_id.eq(mission_id))
                .count()
                .get_result(&mut conn)?;
            Ok(count)
        })
        .await??;

        Ok(count)
    }

    async fn get_one(&self, mission_id: i32, brawler_id: i32) -> Result<MissionModel> {
        use diesel::sql_types::Int4;

        let db_pool = Arc::clone(&self.db_pool);
        let mission = tokio::task::spawn_blocking(move || -> Result<MissionModel> {
            let mut conn = db_pool.get()?;

            let sql = r#"
SELECT
    m.id,
    m.name,
    m.description,
    m.category,
    m.max_crew,
    m.status,
    m.chief_id,
    COALESCE(b.display_name, '') AS chief_display_name,
    COUNT(cm.brawler_id) AS crew_count,
    EXISTS (
        SELECT 1 FROM crew_memberships cm2
        WHERE cm2.mission_id = m.id
          AND cm2.brawler_id = $2
    ) AS is_member,
    m.created_at,
    m.updated_at
FROM missions m
LEFT JOIN brawlers b ON b.id = m.chief_id
LEFT JOIN crew_memberships cm ON cm.mission_id = m.id
WHERE m.id = $1
GROUP BY
    m.id, b.display_name, m.name, m.description, m.category, m.max_crew,
    m.status, m.chief_id, m.created_at, m.updated_at
LIMIT 1
"#;

            let mission = diesel::sql_query(sql)
                .bind::<Int4, _>(mission_id)
                .bind::<Int4, _>(brawler_id)
                .get_result::<MissionModel>(&mut conn)?;
            
            Ok(mission)
        })
        .await??;

        Ok(mission)
    }

    async fn get_all(
        &self,
        mission_filter: &MissionFilter,
        brawler_id: i32,
    ) -> Result<Vec<MissionModel>> {
        use diesel::sql_types::{Nullable, Varchar, Int4};

        let db_pool = Arc::clone(&self.db_pool);
        let mission_filter = mission_filter.clone();

        let rows = tokio::task::spawn_blocking(move || -> Result<Vec<MissionModel>> {
            let mut conn = db_pool.get()?;

            let sql = r#"
SELECT
    m.id,
    m.name,
    m.description,
    m.category,
    m.max_crew,
    m.status,
    m.chief_id,
    COALESCE(b.display_name, '') AS chief_display_name,
    COUNT(cm.brawler_id) AS crew_count,
    EXISTS (
        SELECT 1 FROM crew_memberships cm2
        WHERE cm2.mission_id = m.id
          AND cm2.brawler_id = $3
    ) AS is_member,
    m.created_at,
    m.updated_at
FROM missions m
LEFT JOIN brawlers b ON b.id = m.chief_id
LEFT JOIN crew_memberships cm ON cm.mission_id = m.id
WHERE ($1::varchar IS NULL OR m.status = $1)
  AND ($2::varchar IS NULL OR m.name ILIKE $2)
  AND ($4::varchar IS NULL OR m.category = $4)
  -- AND m.chief_id <> $3
  -- AND NOT EXISTS (
  --  SELECT 1 FROM crew_memberships cm3
  --  WHERE cm3.mission_id = m.id
  --    AND cm3.brawler_id = $3
  -- )
GROUP BY
    m.id, b.display_name, m.name, m.description, m.category, m.max_crew,
    m.status, m.chief_id, m.created_at, m.updated_at
ORDER BY m.created_at DESC
"#;

            let rows = diesel::sql_query(sql)
                .bind::<Nullable<Varchar>, _>(
                    mission_filter.status.as_ref().map(|s| s.to_string()),
                )
                .bind::<Nullable<Varchar>, _>(
                    mission_filter.name.as_ref().map(|n| format!("%{}%", n)),
                )
                .bind::<Int4, _>(brawler_id)
                .bind::<Nullable<Varchar>, _>(
                    mission_filter.category.as_ref().map(|c| c.to_string()),
                )
                .load::<MissionModel>(&mut conn)?;
            
            Ok(rows)
        })
        .await??;

        Ok(rows)
    }

    // *เพิ่ม
    async fn get_joined_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>> {
        use diesel::sql_types::Int4;

        let db_pool = Arc::clone(&self.db_pool);
        let rows = tokio::task::spawn_blocking(move || -> Result<Vec<MissionModel>> {
            let mut conn = db_pool.get()?;

            let sql = r#"
SELECT
    m.id,
    m.name,
    m.description,
    m.category,
    m.max_crew,
    m.status,
    m.chief_id,
    COALESCE(b.display_name, '') AS chief_display_name,
    COUNT(cm.brawler_id) AS crew_count,
    TRUE AS is_member,
    m.created_at,
    m.updated_at
FROM missions m
INNER JOIN crew_memberships cm_join ON m.id = cm_join.mission_id
LEFT JOIN brawlers b ON b.id = m.chief_id
LEFT JOIN crew_memberships cm ON cm.mission_id = m.id
WHERE cm_join.brawler_id = $1
GROUP BY
    m.id, b.display_name, m.name, m.description, m.category, m.max_crew,
    m.status, m.chief_id, m.created_at, m.updated_at
ORDER BY m.created_at DESC
"#;

            let rows = diesel::sql_query(sql)
                .bind::<Int4, _>(brawler_id)
                .load::<MissionModel>(&mut conn)?;
            
            Ok(rows)
        })
        .await??;

        Ok(rows)
    }

    async fn get_crew(&self, mission_id: i32) -> Result<Vec<BrawlerModel>> {
        let db_pool = Arc::clone(&self.db_pool);
        let list = tokio::task::spawn_blocking(move || -> Result<Vec<BrawlerModel>> {
            let mut conn = db_pool.get()?;
            let sql = r#"
SELECT 
    b.id,
    b.display_name,
    COALESCE(b.avatar_url, '') AS avatar_url,
    (
        (SELECT COUNT(cm_s.mission_id)::INTEGER 
        FROM crew_memberships cm_s
        INNER JOIN missions m_s ON cm_s.mission_id = m_s.id
        WHERE cm_s.brawler_id = b.id AND m_s.status = 'Completed')
        +
        (SELECT COUNT(m_s2.id)::INTEGER
        FROM missions m_s2
        WHERE m_s2.chief_id = b.id AND m_s2.status = 'Completed')
    ) AS mission_success_count,
    (
        SELECT COUNT(cm_j.mission_id)::INTEGER 
        FROM crew_memberships cm_j
        WHERE cm_j.brawler_id = b.id
    ) AS mission_join_count
FROM brawlers b
INNER JOIN crew_memberships cm ON b.id = cm.brawler_id
WHERE cm.mission_id = $1
"#;
            let list = diesel::sql_query(sql)
                .bind::<diesel::sql_types::Int4, _>(mission_id)
                .load::<BrawlerModel>(&mut conn)?;
            Ok(list)
        })
        .await??;

        Ok(list)
    }

    // *เพิ่ม
    async fn get_popular_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>> {
        use diesel::sql_types::Int4;

        let db_pool = Arc::clone(&self.db_pool);
        let rows = tokio::task::spawn_blocking(move || -> Result<Vec<MissionModel>> {
            let mut conn = db_pool.get()?;

            let sql = r#"
SELECT
    m.id,
    m.name,
    m.description,
    m.category,
    m.max_crew,
    m.status,
    m.chief_id,
    COALESCE(b.display_name, '') AS chief_display_name,
    COUNT(cm.brawler_id) AS crew_count,
    EXISTS (
        SELECT 1 FROM crew_memberships cm2
        WHERE cm2.mission_id = m.id
          AND cm2.brawler_id = $1
    ) AS is_member,
    m.created_at,
    m.updated_at
FROM missions m
LEFT JOIN brawlers b ON b.id = m.chief_id
LEFT JOIN crew_memberships cm ON cm.mission_id = m.id
WHERE m.status != 'Completed' AND m.status != 'Failed'
GROUP BY
    m.id, b.display_name, m.name, m.description, m.category, m.max_crew,
    m.status, m.chief_id, m.created_at, m.updated_at
ORDER BY crew_count DESC, m.updated_at DESC
LIMIT 6
"#;

            let rows = diesel::sql_query(sql)
                .bind::<Int4, _>(brawler_id)
                .load::<MissionModel>(&mut conn)?;
            
            Ok(rows)
        })
        .await??;

        Ok(rows)
    }
}
