use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::{
    domain::{
        entities::mission_messages::NewMissionMessageEntity,
        repositories::mission_message_repository::MissionMessageRepository,
        value_objects::mission_message_model::MissionMessageModel,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{mission_messages, brawlers},
    },
};

pub struct MissionMessagePostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionMessagePostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionMessageRepository for MissionMessagePostgres {
    async fn create(&self, entity: NewMissionMessageEntity) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        
        diesel::insert_into(mission_messages::table)
            .values(&entity)
            .execute(&mut conn)?;
            
        Ok(())
    }

    async fn get_by_mission_id(&self, mission_id_val: i32) -> Result<Vec<MissionMessageModel>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        
        // Using sql_query to join with brawlers and select into MissionMessageModel
        let sql = r#"
            SELECT 
                mm.id, 
                mm.mission_id, 
                mm.user_id,
                b.display_name as user_display_name,
                b.avatar_url as user_avatar_url,
                mm.content, 
                mm.type as type_,
                mm.created_at
            FROM mission_messages mm
            LEFT JOIN brawlers b ON mm.user_id = b.id
            WHERE mm.mission_id = $1
            ORDER BY mm.created_at ASC
            LIMIT 100
        "#;
        
        let results = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Integer, _>(mission_id_val)
            .load::<MissionMessageModel>(&mut conn)?;
            
        Ok(results)
    }
}
