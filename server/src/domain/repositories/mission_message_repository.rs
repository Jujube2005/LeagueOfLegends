use async_trait::async_trait;
use anyhow::Result;
use crate::domain::{
    entities::mission_messages::NewMissionMessageEntity,
    value_objects::mission_message_model::MissionMessageModel,
};

#[async_trait]
pub trait MissionMessageRepository {
    async fn create(&self, entity: NewMissionMessageEntity) -> Result<()>;
    async fn get_by_mission_id(&self, mission_id: i32) -> Result<Vec<MissionMessageModel>>;
}
