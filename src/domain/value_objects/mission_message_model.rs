use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::prelude::QueryableByName;
use diesel::sql_types::{Integer, Text, Timestamp, Varchar, Nullable};

#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct MissionMessageModel {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub mission_id: i32,
    #[diesel(sql_type = Nullable<Integer>)]
    pub user_id: Option<i32>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub user_display_name: Option<String>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub user_avatar_url: Option<String>,
    #[diesel(sql_type = Text)]
    pub content: String,
    #[diesel(sql_type = Varchar)]
    pub type_: String,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
}
