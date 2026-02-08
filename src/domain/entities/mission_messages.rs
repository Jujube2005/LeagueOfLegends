use crate::infrastructure::database::schema::mission_messages;
use chrono::NaiveDateTime;
use diesel::{Selectable, Queryable, Identifiable, Insertable, Associations};
use crate::domain::entities::brawlers::BrawlerEntity;
use crate::domain::entities::missions::MissionEntity;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Associations)]
#[diesel(belongs_to(BrawlerEntity, foreign_key = user_id))]
#[diesel(belongs_to(MissionEntity, foreign_key = mission_id))]
#[diesel(table_name = mission_messages)]
pub struct MissionMessageEntity {
    pub id: i32,
    pub mission_id: i32,
    pub user_id: Option<i32>,
    pub content: String,
    // Note: In schema it is defined as type_, diesel maps it automatically if name matches
    #[diesel(column_name = type_)] 
    pub type_: String, 
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = mission_messages)]
pub struct NewMissionMessageEntity {
    pub mission_id: i32,
    pub user_id: Option<i32>,
    pub content: String,
    #[diesel(column_name = type_)]
    pub type_: String, 
}
