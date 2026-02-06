use crate::domain::{
    entities::{crew_memberships::CrewMemberShips, notification::{Notification, NotificationType}},
    repositories::{
        crew_operation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
        AchievementRepository, BrawlerRepository,
    },
    services::notification_service::NotificationService,
    value_objects::mission_statuses::MissionStatuses,
};
use anyhow::Result;
use std::sync::Arc;

pub struct CrewOperationUseCase<T1, T2, T3, T4>
where
    T1: CrewOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    crew_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
    achievement_repository: Arc<T3>,
    brawler_repository: Arc<T4>,
    notification_service: Arc<dyn NotificationService>,
}

impl<T1, T2, T3, T4> CrewOperationUseCase<T1, T2, T3, T4>
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    pub fn new(
        crew_operation_repository: Arc<T1>, 
        mission_viewing_repository: Arc<T2>,
        achievement_repository: Arc<T3>,
        brawler_repository: Arc<T4>,
        notification_service: Arc<dyn NotificationService>,
    ) -> Self {
        Self {
            crew_operation_repository,
            mission_viewing_repository,
            achievement_repository,
            brawler_repository,
            notification_service,
        }
    }

    pub async fn join(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let max_crew_per_mission = std::env::var("MAX_CREW_PER_MISSION")
            .expect("missing value")
            .parse()?;

        let mission = self.mission_viewing_repository.get_one(mission_id, brawler_id).await?;

        // หัวหน้าห้ามจอย
        if mission.chief_id == brawler_id {
            return Err(anyhow::anyhow!(
                "The Chief can not join in his own mission as a crew member!!"
            ));
        }

        // เช็คว่าเข้าซ้ำมั้ย *เพิ่ม
        let is_joined = self
            .crew_operation_repository
            .is_member(mission_id,brawler_id)
            .await?;
        if is_joined {
            return Err(anyhow::anyhow!("Already joined"));
        }

        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;

        // เช็คสถานะ
        let mission_status_condition = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();
        if !mission_status_condition {
            return Err(anyhow::anyhow!("Mission is not joinable"));
        }

        // คนเต็ม
        let crew_count_condition = crew_count < max_crew_per_mission;
        if !crew_count_condition {
            return Err(anyhow::anyhow!("Mission is full"));
        }

        self.crew_operation_repository
            .join(CrewMemberShips {
                mission_id,
                brawler_id,
            })
            .await?;

        // Notification: Notify Chief
        let notification = Notification {
            recipient_id: Some(mission.chief_id),
            title: "New Crew Member".to_string(),
            message: format!("Someone joined your mission: {}", mission.name),
            notification_type: NotificationType::JoinMission,
            metadata: serde_json::json!({
                "mission_id": mission_id,
                "joiner_id": brawler_id
            }),
        };
        let _ = self.notification_service.send(notification).await;

        // Check Achievements
        if let Ok(brawler) = self.brawler_repository.find_by_id(brawler_id).await {
            let _ = self.achievement_repository.check_and_award(brawler_id, "mission_join", brawler.mission_join_count).await;
        }

        Ok(())
    }

    pub async fn leave(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        self.crew_operation_repository
            .leave(CrewMemberShips {
                mission_id,
                brawler_id,
            })
            .await
    }

    pub async fn kick_crew(&self, mission_id: i32, chief_id: i32, member_id: i32) -> Result<()> {
        let mission = self.mission_viewing_repository.get_one(mission_id, chief_id).await?;

        if mission.chief_id != chief_id {
            return Err(anyhow::anyhow!("Only the Chief can kick members"));
        }

        self.crew_operation_repository
            .leave(CrewMemberShips {
                mission_id,
                brawler_id: member_id,
            })
            .await?;

        // Notification: Notify Kicked Member
        let notification = Notification {
            recipient_id: Some(member_id),
            title: "You have been kicked".to_string(),
            message: format!("You were kicked from mission: {}", mission.name),
            notification_type: NotificationType::MissionStatusUpdate, 
            metadata: serde_json::json!({ "mission_id": mission_id }),
        };
        let _ = self.notification_service.send(notification).await;

        Ok(())
    }
}
