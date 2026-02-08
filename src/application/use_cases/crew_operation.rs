use crate::domain::{
    entities::{
        crew_memberships::CrewMemberShips, 
        notification::{Notification, NotificationType},
        mission_messages::NewMissionMessageEntity,
    },
    repositories::{
        crew_operation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
        AchievementRepository, BrawlerRepository, mission_message_repository::MissionMessageRepository,
    },
    services::notification_service::NotificationService,
    value_objects::mission_statuses::MissionStatuses,
};
use anyhow::Result;
use std::sync::Arc;


use crate::application::services::mission_realtime::{MissionRealtimeService, ChatMessage};

pub struct CrewOperationUseCase<T1, T2, T3, T4, T5>
where
    T1: CrewOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
    T5: MissionMessageRepository + Send + Sync,
{
    crew_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
    achievement_repository: Arc<T3>,
    brawler_repository: Arc<T4>,
    mission_message_repository: Arc<T5>,
    notification_service: Arc<dyn NotificationService>,
    realtime_service: Arc<MissionRealtimeService>,
}

impl<T1, T2, T3, T4, T5> CrewOperationUseCase<T1, T2, T3, T4, T5>
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
    T5: MissionMessageRepository + Send + Sync,
{
    pub fn new(
        crew_operation_repository: Arc<T1>, 
        mission_viewing_repository: Arc<T2>,
        achievement_repository: Arc<T3>,
        brawler_repository: Arc<T4>,
        mission_message_repository: Arc<T5>,
        notification_service: Arc<dyn NotificationService>,
        realtime_service: Arc<MissionRealtimeService>,
    ) -> Self {
        Self {
            crew_operation_repository,
            mission_viewing_repository,
            achievement_repository,
            brawler_repository,
            mission_message_repository,
            notification_service,
            realtime_service,
        }
    }
    
    async fn log_and_broadcast_system_message(&self, mission_id: i32, content: String) {
        let entity = NewMissionMessageEntity {
            mission_id,
            user_id: None,
            content: content.clone(),
            type_: "system".to_string(),
        };

        // Persist
        let _ = self.mission_message_repository.create(entity).await;

        // Broadcast
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

    pub async fn join(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
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
            || mission.status == MissionStatuses::Failed.to_string()
            || mission.status == MissionStatuses::InProgress.to_string();
        if !mission_status_condition {
            return Err(anyhow::anyhow!("Mission is not joinable"));
        }
        // คนเต็ม
        let crew_count_condition = (crew_count as i32) < mission.max_crew;
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

        // Check Achievements AND Broadcast
        if let Ok(brawler) = self.brawler_repository.find_by_id(brawler_id).await {
            // Log for debug
            println!("DEBUG: Brawler {} join_count is {}", brawler.display_name, brawler.mission_join_count);

            if let Ok(awarded) = self.achievement_repository.check_and_award(brawler_id, "mission_join", brawler.mission_join_count).await {
                for name in awarded {
                    self.log_and_broadcast_system_message(mission_id, format!("{} earned achievement: {}", brawler.display_name, name)).await;
                }
            }
            
            // Broadcast join
            self.log_and_broadcast_system_message(mission_id, format!("{} joined the mission", brawler.display_name)).await;
        } else {
             self.log_and_broadcast_system_message(mission_id, "A new member joined the mission".to_string()).await;
        }

        Ok(())
    }

    pub async fn leave(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let result = self.crew_operation_repository
            .leave(CrewMemberShips {
                mission_id,
                brawler_id,
            })
            .await;
            
        // Broadcast leave
        if result.is_ok() {
             if let Ok(brawler) = self.brawler_repository.find_by_id(brawler_id).await {
                self.log_and_broadcast_system_message(mission_id, format!("{} left the mission", brawler.display_name)).await;
            } else {
                self.log_and_broadcast_system_message(mission_id, "A member left the mission".to_string()).await;
            }
        }
        
        result
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

        // Broadcast kick
         if let Ok(brawler) = self.brawler_repository.find_by_id(member_id).await {
            self.log_and_broadcast_system_message(mission_id, format!("{} was kicked from the mission", brawler.display_name)).await;
        } else {
            self.log_and_broadcast_system_message(mission_id, "A member was kicked from the mission".to_string()).await;
        }

        Ok(())
    }
}
