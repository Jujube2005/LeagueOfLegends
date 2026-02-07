use std::sync::Arc;
use anyhow::Result;
use crate::domain::{
    entities::mission_messages::NewMissionMessageEntity,
    repositories::mission_message_repository::MissionMessageRepository,
    value_objects::mission_message_model::MissionMessageModel,
};

pub struct MissionChatUseCase<T> {
    repository: Arc<T>,
}

impl<T> MissionChatUseCase<T>
where
    T: MissionMessageRepository + Send + Sync,
{
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn get_messages(&self, mission_id: i32) -> Result<Vec<MissionMessageModel>> {
        self.repository.get_by_mission_id(mission_id).await
    }

    pub async fn send_message(&self, mission_id: i32, user_id: i32, content: String) -> Result<()> {
        let entity = NewMissionMessageEntity {
            mission_id,
            user_id: Some(user_id),
            content,
            type_: "chat".to_string(),
        };

        self.repository.create(entity).await.map(|_| ())
    }
}
