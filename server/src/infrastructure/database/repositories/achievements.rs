use crate::domain::{
    entities::achievements::{Achievement, BrawlerAchievement},
    repositories::achievements::AchievementRepository,
};
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::schema::{achievements, brawler_achievements};
use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

pub struct AchievementRepositoryImpl {
    pub pool: Arc<PgPoolSquad>,
}

impl AchievementRepositoryImpl {
    pub fn new(pool: Arc<PgPoolSquad>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AchievementRepository for AchievementRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<Achievement>> {
        let mut conn = self.pool.get()?;
        let result = achievements::table
            .load::<Achievement>(&mut conn)?;
        Ok(result)
    }

    async fn get_by_brawler_id(&self, brawler_id: i32) -> Result<Vec<(Achievement, Option<BrawlerAchievement>)>> {
        let mut conn = self.pool.get()?;
        
        // This is a LEFT JOIN to get all achievements and mark which ones the user has
        let result = achievements::table
            .left_join(brawler_achievements::table.on(
                brawler_achievements::achievement_id.eq(achievements::id)
                .and(brawler_achievements::brawler_id.eq(brawler_id))
            ))
            .select((achievements::all_columns, brawler_achievements::all_columns.nullable()))
            .load::<(Achievement, Option<BrawlerAchievement>)>(&mut conn)?;
            
        Ok(result)
    }

    async fn award_achievement(&self, brawler_id_val: i32, achievement_id_val: i32) -> Result<()> {
        let mut conn = self.pool.get()?;
        
        diesel::insert_into(brawler_achievements::table)
            .values((
                brawler_achievements::brawler_id.eq(brawler_id_val),
                brawler_achievements::achievement_id.eq(achievement_id_val),
            ))
            .on_conflict_do_nothing()
            .execute(&mut conn)?;
            
        Ok(())
    }

    async fn check_and_award(&self, brawler_id_val: i32, condition_type_val: &str, current_value: i32) -> Result<Vec<String>> {
        let mut conn = self.pool.get()?;
        
        // 1. Find potential achievements
        let potential_achievements = achievements::table
            .filter(achievements::condition_type.eq(condition_type_val))
            .filter(achievements::condition_value.le(current_value))
            .load::<Achievement>(&mut conn)?;
            
        let mut awarded_names = Vec::new();

        for achievement in potential_achievements {
            // 2. Try to insert (award)
            let rows_inserted = diesel::insert_into(brawler_achievements::table)
                .values((
                    brawler_achievements::brawler_id.eq(brawler_id_val),
                    brawler_achievements::achievement_id.eq(achievement.id),
                ))
                .on_conflict_do_nothing()
                .execute(&mut conn)?;
            
            if rows_inserted > 0 {
                awarded_names.push(achievement.name);
            }
        }

        Ok(awarded_names)
    }
}
