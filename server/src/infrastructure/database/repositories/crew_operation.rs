use anyhow::Result;
use async_trait::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl, dsl::{delete, exists}, insert_into, query_dsl::methods::FilterDsl, select};
use std::sync::Arc;

use crate::{
    domain::{
        entities::crew_memberships::CrewMemberShips,
        repositories::crew_operation::CrewOperationRepository,
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::{crew_memberships, brawlers}},
};

pub struct CrewOperationPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewOperationPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewOperationRepository for CrewOperationPostgres {
    async fn join(&self, crew_member_ships: CrewMemberShips) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let brawler_id = crew_member_ships.brawler_id;
        let result = insert_into(crew_memberships::table)
            .values(crew_member_ships)
            .execute(&mut conn);

        match result {
            Ok(_) => {
                diesel::update(brawlers::table)
                    .filter(brawlers::id.eq(brawler_id))
                    .set(brawlers::mission_join_count.eq(brawlers::mission_join_count + 1))
                    .execute(&mut conn)?;
                Ok(())
            },
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => Err(anyhow::anyhow!("Already joined")),
            Err(e) => {
                let err_msg = e.to_string();
                println!("Database Insert Error: {}", err_msg);
                if err_msg.contains("duplicate key") || err_msg.contains("UniqueViolation") {
                     Err(anyhow::anyhow!("Already joined"))
                } else {
                     Err(e.into())
                }
            },
        }
    }

    async fn leave(&self, crew_member_ships: CrewMemberShips) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_member_ships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_member_ships.mission_id))
            .execute(&mut conn)?;
        Ok(())
    }

    // *เพิ่ม
    async fn is_member(&self, mission_id: i32, brawler_id: i32) -> Result<bool> {
    let mut conn = Arc::clone(&self.db_pool).get()?;

    let result = select(exists(
        crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .filter(crew_memberships::brawler_id.eq(brawler_id)),
    ))
    .get_result::<bool>(&mut conn)?;

    Ok(result)
}
}
