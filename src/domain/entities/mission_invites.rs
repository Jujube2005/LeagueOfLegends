use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable, Associations};
use crate::infrastructure::database::schema::mission_invites;
use crate::domain::entities::missions::MissionEntity;
use crate::domain::entities::brawlers::BrawlerEntity;

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, Associations)]
#[diesel(belongs_to(MissionEntity, foreign_key = mission_id))]
#[diesel(belongs_to(BrawlerEntity, foreign_key = user_id))]
pub struct MissionInvite {
    pub id: i32,
    pub mission_id: i32,
    pub user_id: i32,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = mission_invites)]
pub struct NewMissionInvite {
    pub mission_id: i32,
    pub user_id: i32,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct MissionInviteDetails {
    pub id: i32,
    pub mission_id: i32,
    pub mission_name: String,
    pub chief_name: String,
    pub status: String,
}
