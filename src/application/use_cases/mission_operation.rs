use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    entities::{
        notification::{Notification, NotificationType},
        mission_messages::NewMissionMessageEntity,
    },
    repositories::{
        mission_operation::MissionOperationRepository, mission_viewing::MissionViewingRepository,
        AchievementRepository, BrawlerRepository, mission_message_repository::MissionMessageRepository,
    },
    services::notification_service::NotificationService,
    value_objects::mission_statuses::MissionStatuses,
};

use crate::application::services::mission_realtime::{MissionRealtimeService, ChatMessage};

pub struct MissionOperationUseCase<T1, T2, T3, T4, T5>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
    T5: MissionMessageRepository + Send + Sync,
{
    mission_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
    achievement_repository: Arc<T3>,
    brawler_repository: Arc<T4>,
    mission_message_repository: Arc<T5>,
    notification_service: Arc<dyn NotificationService>,
    realtime_service: Arc<MissionRealtimeService>,
}

impl<T1, T2, T3, T4, T5> MissionOperationUseCase<T1, T2, T3, T4, T5>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
    T5: MissionMessageRepository + Send + Sync,
{
    pub fn new(
        mission_operation_repository: Arc<T1>,
        mission_viewing_repository: Arc<T2>,
        achievement_repository: Arc<T3>,
        brawler_repository: Arc<T4>,
        mission_message_repository: Arc<T5>,
        notification_service: Arc<dyn NotificationService>,
        realtime_service: Arc<MissionRealtimeService>,
    ) -> Self {
        Self {
            mission_operation_repository,
            mission_viewing_repository,
            achievement_repository,
            brawler_repository,
            mission_message_repository,
            notification_service,
            realtime_service,
        }
    }

    async fn notify_crew(&self, mission_id: i32, title: &str, message: &str) -> Result<()> {
        let crew = self.mission_viewing_repository.get_crew(mission_id).await?;
        for member in crew {
            let notification = Notification {
                recipient_id: Some(member.id),
                title: title.to_string(),
                message: message.to_string(),
                notification_type: NotificationType::MissionStatusUpdate,
                metadata: serde_json::json!({ "mission_id": mission_id }),
            };
            let _ = self.notification_service.send(notification).await;
        }
        Ok(())
    }

    async fn log_and_broadcast_status_change(&self, mission_id: i32, content: String) {
        let entity = NewMissionMessageEntity {
            mission_id,
            user_id: None,
            content: content.clone(),
            type_: "system".to_string(),
        };

        // Persist to DB
        let _ = self.mission_message_repository.create(entity).await;

        // Broadcast to WebSockets
        let msg = ChatMessage {
            mission_id,
            user_id: None,
            user_display_name: None,
            user_avatar_url: None,
            content,
            type_: "system".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        self.realtime_service.broadcast(mission_id, msg);
    }

    pub async fn in_progress(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self.mission_viewing_repository.get_one(mission_id, chief_id).await?;

        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;

        let is_status_open_or_fail = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();

        let max_crew_per_mission: i64 = std::env::var("MAX_CREW_PER_MISSION")
            .expect("missing value")
            .parse()?;

        let update_condition = is_status_open_or_fail
            && crew_count > 0
            && crew_count < max_crew_per_mission
            && mission.chief_id == chief_id;
        if !update_condition {
            return Err(anyhow::anyhow!("Invalid condition to change stages!"));
        }

        let result = self
            .mission_operation_repository
            .to_progress(mission_id, chief_id)
            .await?;
        
        self.notify_crew(
            mission_id,
            "Mission Started",
            &format!("Mission '{}' is now In Progress!", mission.name),
        )
        .await?;

        self.log_and_broadcast_status_change(mission_id, format!("Mission started: {}", mission.name)).await;

        Ok(result)
    }
    pub async fn to_completed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self.mission_viewing_repository.get_one(mission_id, chief_id).await?;

        let update_condition = mission.status == MissionStatuses::InProgress.to_string()
            && mission.chief_id == chief_id;
        if !update_condition {
            return Err(anyhow::anyhow!("Invalid condition to change stages!"));
        }

        // Get crew before completion (to know who to award)
        let crew_members = self.mission_viewing_repository.get_crew(mission_id).await?;

        let result = self
            .mission_operation_repository
            .to_completed(mission_id, chief_id)
            .await?;

        self.notify_crew(
            mission_id,
            "Mission Completed",
            &format!("Mission '{}' has been completed!", mission.name),
        )
        .await?;

        self.log_and_broadcast_status_change(mission_id, format!("Mission completed: {}", mission.name)).await;

        // Award achievements
        // 1. Chief
        if let Ok(chief) = self.brawler_repository.find_by_id(chief_id).await {
            // Log for debug
            println!("DEBUG: Chief {} success_count is {}", chief.display_name, chief.mission_success_count);
            
            if let Ok(awarded) = self.achievement_repository.check_and_award(chief_id, "mission_complete", chief.mission_success_count).await {
                for name in awarded {
                     self.log_and_broadcast_status_change(mission_id, format!("{} earned achievement: {}", chief.display_name, name)).await;
                }
            }
        }

        // 2. Crew
        for member in crew_members {
             if let Ok(brawler) = self.brawler_repository.find_by_id(member.id).await {
                println!("DEBUG: Crew member {} success_count is {}", brawler.display_name, brawler.mission_success_count);
                
                if let Ok(awarded) = self.achievement_repository.check_and_award(member.id, "mission_complete", brawler.mission_success_count).await {
                    for name in awarded {
                         self.log_and_broadcast_status_change(mission_id, format!("{} earned achievement: {}", brawler.display_name, name)).await;
                    }
                }
            }
        }

        Ok(result)
    }

    pub async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mission = self.mission_viewing_repository.get_one(mission_id, chief_id).await?;

        let update_condition = mission.status == MissionStatuses::InProgress.to_string()
            && mission.chief_id == chief_id;
        if !update_condition {
            return Err(anyhow::anyhow!("Invalid condition to change stages!"));
        }
        let result = self
            .mission_operation_repository
            .to_failed(mission_id, chief_id)
            .await?;

        self.notify_crew(
            mission_id,
            "Mission Failed",
            &format!("Mission '{}' has failed.", mission.name),
        )
        .await?;

        self.log_and_broadcast_status_change(mission_id, format!("Mission failed: {}", mission.name)).await;

        Ok(result)
    }
}
