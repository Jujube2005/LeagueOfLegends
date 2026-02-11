// *เพิ่ม

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::achievements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Achievement {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub condition_type: Option<String>,
    pub condition_value: Option<i32>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(super::brawlers::BrawlerEntity, foreign_key = brawler_id))]
#[diesel(belongs_to(Achievement, foreign_key = achievement_id))]
#[diesel(table_name = crate::schema::brawler_achievements)]
#[diesel(primary_key(brawler_id, achievement_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BrawlerAchievement {
    pub brawler_id: i32,
    pub achievement_id: i32,
    pub earned_at: NaiveDateTime,
}
