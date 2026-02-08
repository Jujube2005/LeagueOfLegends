use async_trait::async_trait;
use diesel::{
    ExpressionMethods, RunQueryDsl, delete, insert_into
};
use std::sync::Arc;
use crate::{domain::{entities::crew_memberships::CrewMemberShips, repositories::crew_operation::CrewOperationRepository}, infrastructure::database::{postgresql_connection::PgPoolSquad, schema::{brawlers, crew_memberships}}};
use anyhow::Result;
use diesel::prelude::*;

pub struct CrewPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}


#[async_trait]
impl CrewOperationRepository for CrewPostgres {


    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        let brawler_id = crew_memberships.brawler_id;
        
        connection.transaction::<_, diesel::result::Error, _>(|conn| {
            insert_into(crew_memberships::table)
                .values(&crew_memberships)
                .execute(conn)?;

            diesel::update(brawlers::table)
                .filter(brawlers::id.eq(brawler_id))
                .set(brawlers::mission_join_count.eq(brawlers::mission_join_count + 1))
                .execute(conn)?;
            Ok(())
        })?;

        Ok(())
    }

    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(&mut connection)?;
        Ok(())
    }

    async fn is_member(&self, mission_id: i32, brawler_id: i32) -> Result<bool> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        let count = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .filter(crew_memberships::brawler_id.eq(brawler_id))
            .count()
            .get_result::<i64>(&mut connection)?;
        
        Ok(count > 0)
    }
}