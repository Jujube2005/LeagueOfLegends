use std::sync::Arc;
use anyhow::Result;
use crate::domain::{
    entities::mission_messages::NewMissionMessageEntity,
    repositories::mission_message_repository::MissionMessageRepository,
    value_objects::mission_message_model::MissionMessageModel,
};

use crate::application::services::mission_realtime::{MissionRealtimeService, ChatMessage};

pub struct MissionChatUseCase<T> {
    repository: Arc<T>,
    realtime_service: Arc<MissionRealtimeService>,
}

impl<T> MissionChatUseCase<T>
where
    T: MissionMessageRepository + Send + Sync,
{
    pub fn new(repository: Arc<T>, realtime_service: Arc<MissionRealtimeService>) -> Self {
        Self { repository, realtime_service }
    }

    pub async fn get_messages(&self, mission_id: i32) -> Result<Vec<MissionMessageModel>> {
        self.repository.get_by_mission_id(mission_id).await
    }

    pub async fn send_message(&self, mission_id: i32, user_id: i32, content: String) -> Result<()> {
        let entity = NewMissionMessageEntity {
            mission_id,
            user_id: Some(user_id),
            content: content.clone(),
            type_: "chat".to_string(),
        };

        let _msg_id = self.repository.create(entity).await?;

        // Broadcast
        // In a real app we would fetch the user details to broadcast name/avatar
        // For now we send what we have, frontend might need to refetch or we accept missing data
        let broadcast_msg = ChatMessage {
            mission_id,
            user_id: Some(user_id),
            user_display_name: None, // TODO: Fetch user details
            user_avatar_url: None,
            content,
            type_: "chat".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        
        self.realtime_service.broadcast(mission_id, broadcast_msg);

        Ok(())
    }
}
