// *เพิ่ม

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AchievementViewModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub is_earned: bool,
    pub earned_at: Option<NaiveDateTime>,
}
