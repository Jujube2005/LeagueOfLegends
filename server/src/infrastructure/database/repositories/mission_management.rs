use crate::{
    domain::{
        entities::missions::{AddMissionEntity, EditMissionEntity},
        repositories::mission_management::MissionManagementRepository,
        value_objects::mission_statuses::MissionStatuses,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{crew_memberships, missions},
    },
};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{
    dsl::{delete, insert_into, now, update},
    Connection, ExpressionMethods, RunQueryDsl,
};
use std::sync::Arc;

pub struct MissionManagementPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionManagementPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionManagementRepository for MissionManagementPostgres {
    async fn add(&self, add_mission_entity: AddMissionEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(missions::table)
            .values(add_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn edit(&self, mission_id: i32, edit_mission_entity: EditMissionEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = update(missions::table)
            .filter(missions::id.eq(mission_id))
            // *เพิ่ม
            .filter(missions::chief_id.eq(edit_mission_entity.chief_id))
            .filter(missions::deleted_at.is_null())
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .set(edit_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn transfer_ownership(&self, mission_id: i32, current_chief_id: i32, new_chief_id: i32) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        conn.transaction::<_, anyhow::Error, _>(|conn| {
            // 1. Remove new chief from crew if present
            delete(crew_memberships::table)
                .filter(crew_memberships::mission_id.eq(mission_id))
                .filter(crew_memberships::brawler_id.eq(new_chief_id))
                .execute(conn)?;

            // 2. Add current chief to crew
            insert_into(crew_memberships::table)
                .values((
                    crew_memberships::mission_id.eq(mission_id),
                    crew_memberships::brawler_id.eq(current_chief_id),
                ))
                .execute(conn)?;

            // 3. Update mission chief
            update(missions::table)
                .filter(missions::id.eq(mission_id))
                .filter(missions::chief_id.eq(current_chief_id))
                .set(missions::chief_id.eq(new_chief_id))
                .execute(conn)?;

            Ok(())
        })?;

        Ok(())
    }

    async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        update(missions::table)
            .filter(missions::id.eq(mission_id))
            // *เพิ่ม
            .filter(missions::chief_id.eq(chief_id))
            .filter(missions::deleted_at.is_null())
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .set(missions::deleted_at.eq(now))
            .execute(&mut conn)?;

        Ok(())
    }
}
