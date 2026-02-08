use std::sync::Arc;
use anyhow::Result;
use crate::domain::{
    repositories::AchievementRepository,
    value_objects::achievement_model::AchievementViewModel,
};

pub struct AchievementUseCase<T>
where
    T: AchievementRepository + Send + Sync,
{
    repository: Arc<T>,
}

impl<T> AchievementUseCase<T>
where
    T: AchievementRepository + Send + Sync,
{
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn get_my_achievements(&self, brawler_id: i32) -> Result<Vec<AchievementViewModel>> {
        let data = self.repository.get_by_brawler_id(brawler_id).await?;
        
        let view_models = data.into_iter().map(|(achievement, brawler_achievement)| {
            AchievementViewModel {
                id: achievement.id,
                name: achievement.name,
                description: achievement.description,
                icon_url: achievement.icon_url,
                is_earned: brawler_achievement.is_some(),
                earned_at: brawler_achievement.map(|ba| ba.earned_at),
            }
        }).collect();
        
        Ok(view_models)
    }
}
