use crate::domain::{
    entities::{
        mission_invites::{MissionInviteDetails, MissionInvite, NewMissionInvite},
        crew_memberships::CrewMemberShips,
        mission_messages::NewMissionMessageEntity,
    },
    repositories::{
        mission_invites::MissionInviteRepository,
        mission_viewing::MissionViewingRepository,
        crew_operation::CrewOperationRepository,
        mission_message_repository::MissionMessageRepository,
        brawlers::BrawlerRepository,
        achievements::AchievementRepository,
    },
};
use anyhow::{anyhow, Result};
use std::sync::Arc;

use crate::application::services::mission_realtime::{MissionRealtimeService, ChatMessage};

pub struct MissionInviteUseCase {
    invite_repo: Arc<dyn MissionInviteRepository>,
    mission_repo: Arc<dyn MissionViewingRepository>,
    crew_repo: Arc<dyn CrewOperationRepository>,
    brawler_repo: Arc<dyn BrawlerRepository>,
    message_repo: Arc<dyn MissionMessageRepository>,
    achievement_repo: Arc<dyn AchievementRepository>,
    realtime_service: Arc<MissionRealtimeService>,
}

impl MissionInviteUseCase {
    pub fn new(
        invite_repo: Arc<dyn MissionInviteRepository>,
        mission_repo: Arc<dyn MissionViewingRepository>,
        crew_repo: Arc<dyn CrewOperationRepository>,
        brawler_repo: Arc<dyn BrawlerRepository>,
        message_repo: Arc<dyn MissionMessageRepository>,
        achievement_repo: Arc<dyn AchievementRepository>,
        realtime_service: Arc<MissionRealtimeService>,
    ) -> Self {
        Self {
            invite_repo,
            mission_repo,
            crew_repo,
            brawler_repo,
            message_repo,
            achievement_repo,
            realtime_service,
        }
    }

    fn broadcast_system_message(&self, mission_id: i32, content: String) {
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

    pub async fn invite(&self, mission_id: i32, inviter_id: i32, user_id: i32) -> Result<MissionInvite> {
        let max_crew = self.mission_repo.crew_counting(mission_id).await?;
        // Simplified check: assume invite logic is handled after proper checks
        
        if self.crew_repo.is_member(user_id, mission_id).await? {
             return Err(anyhow::anyhow!("User is already a member"));
        }
        
        if self.invite_repo.check_exists(mission_id, user_id).await? {
            return Err(anyhow::anyhow!("User is already invited"));
        }

        let invite = self.invite_repo.create(NewMissionInvite {
            mission_id,
            user_id,
            status: "pending".to_string(),
        }).await?;

        // Broadcast invite if needed
        if let Ok(brawler) = self.brawler_repo.find_by_id(user_id).await {
            self.broadcast_system_message(mission_id, format!("{} was invited to the mission", brawler.username));
        }

        Ok(invite)
    }

    pub async fn accept(&self, invite_id: i32, user_id: i32) -> Result<()> {
        let invite = self.invite_repo.find_by_id(invite_id).await?
            .ok_or_else(|| anyhow::anyhow!("Invite not found"))?;

        if invite.user_id != user_id {
            return Err(anyhow::anyhow!("Not authorized to accept this invite"));
        }
        if invite.status != "pending" {
            return Err(anyhow::anyhow!("Invite is not pending"));
        }

        let mission = self.mission_repo.get_one(invite.mission_id, user_id).await?;
        if mission.crew_count >= mission.max_crew as i64 {
            return Err(anyhow::anyhow!("Mission is full"));
        }

        // Add member
        self.crew_repo.join(CrewMemberShips {
            mission_id: invite.mission_id,
            brawler_id: user_id,
        }).await?;

        // Update invite status
        self.invite_repo.update_status(invite_id, "accepted".to_string()).await?;

        // Update stats
        // self.brawler_repo.increment_join_count(user_id).await?; // Done in crew_repo.join
        if let Ok(brawler) = self.brawler_repo.find_by_id(user_id).await {
             let _ = self.achievement_repo.check_and_award(user_id, "mission_join", brawler.mission_join_count).await;
        }

        // System Message
        let brawler = self.brawler_repo.find_by_id(user_id).await?; // Assuming it returns BrawlerEntity directly on success
        let msg_content = format!("{} joined the mission via invite", brawler.username);
        let msg = NewMissionMessageEntity {
            mission_id: invite.mission_id,
            user_id: None,
            content: msg_content.clone(),
            type_: "system".to_string(),
        };
        self.message_repo.create(msg).await?;

        // Broadcast accept
        self.broadcast_system_message(invite.mission_id, msg_content);

        Ok(())
    }

    pub async fn decline(&self, invite_id: i32, user_id: i32) -> Result<()> {
        let invite = self.invite_repo.find_by_id(invite_id).await?
            .ok_or_else(|| anyhow::anyhow!("Invite not found"))?;

        if invite.user_id != user_id {
            return Err(anyhow::anyhow!("Not authorized to decline this invite"));
        }
        
        self.invite_repo.update_status(invite_id, "rejected".to_string()).await?;
        Ok(())
    }

    pub async fn get_my_pending_invites(&self, user_id: i32) -> Result<Vec<MissionInviteDetails>> {
        self.invite_repo.find_invites_details_by_user(user_id).await
    }
}
