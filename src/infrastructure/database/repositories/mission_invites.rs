use crate::{
    domain::{
        entities::mission_invites::{MissionInvite, NewMissionInvite, MissionInviteDetails},
        repositories::mission_invites::MissionInviteRepository,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{mission_invites, missions, brawlers},
    },
};
use anyhow::Result;
use async_trait::async_trait;
use diesel::{
    dsl::{insert_into, update},
    ExpressionMethods, RunQueryDsl, QueryDsl, OptionalExtension, JoinOnDsl
};
use std::sync::Arc;

pub struct MissionInvitePostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionInvitePostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionInviteRepository for MissionInvitePostgres {
    async fn create(&self, invite: NewMissionInvite) -> Result<MissionInvite> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(mission_invites::table)
            .values(invite)
            .get_result::<MissionInvite>(&mut conn)?;
        Ok(result)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<MissionInvite>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = mission_invites::table
            .find(id)
            .first::<MissionInvite>(&mut conn)
            .optional()?;
        Ok(result)
    }

    async fn find_invites_by_user(&self, user_id: i32) -> Result<Vec<MissionInvite>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = mission_invites::table
            .filter(mission_invites::user_id.eq(user_id))
            .filter(mission_invites::status.eq("pending"))
            .load::<MissionInvite>(&mut conn)?;
        Ok(results)
    }

    async fn find_invites_details_by_user(&self, user_id: i32) -> Result<Vec<MissionInviteDetails>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = mission_invites::table
            .inner_join(missions::table.inner_join(brawlers::table))
            .filter(mission_invites::user_id.eq(user_id))
            .filter(mission_invites::status.eq("pending"))
            .select((
                mission_invites::id,
                mission_invites::mission_id,
                missions::name,
                brawlers::display_name,
                mission_invites::status,
            ))
            .load::<MissionInviteDetails>(&mut conn)?;
        
        Ok(results)
    }

    async fn find_invites_by_mission(&self, mission_id: i32) -> Result<Vec<MissionInvite>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = mission_invites::table
            .filter(mission_invites::mission_id.eq(mission_id))
            .load::<MissionInvite>(&mut conn)?;
        Ok(results)
    }

    async fn update_status(&self, id: i32, status: String) -> Result<MissionInvite> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = update(mission_invites::table)
            .filter(mission_invites::id.eq(id))
            .set(mission_invites::status.eq(status))
            .get_result::<MissionInvite>(&mut conn)?;
        Ok(result)
    }

    async fn check_exists(&self, mission_id: i32, user_id: i32) -> Result<bool> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let exists = diesel::select(diesel::dsl::exists(
            mission_invites::table
                .filter(mission_invites::mission_id.eq(mission_id))
                .filter(mission_invites::user_id.eq(user_id))
                .filter(mission_invites::status.eq("pending")),
        ))
        .get_result(&mut conn)?;
        
        Ok(exists)
    }
}
