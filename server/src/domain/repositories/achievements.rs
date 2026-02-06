use crate::domain::entities::achievements::{Achievement, BrawlerAchievement};
use async_trait::async_trait;
use anyhow::Result;


#[async_trait]
pub trait AchievementRepository {
    async fn get_all(&self) -> Result<Vec<Achievement>>;
    async fn get_by_brawler_id(&self, brawler_id: i32) -> Result<Vec<(Achievement, Option<BrawlerAchievement>)>>;
    async fn award_achievement(&self, brawler_id: i32, achievement_id: i32) -> Result<()>;
    async fn check_and_award(&self, brawler_id: i32, condition_type: &str, current_value: i32) -> Result<Vec<String>>;
}
