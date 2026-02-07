use crate::domain::entities::mission_invites::{MissionInvite, NewMissionInvite, MissionInviteDetails};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MissionInviteRepository: Send + Sync {
    async fn create(&self, invite: NewMissionInvite) -> Result<MissionInvite>;
    async fn find_by_id(&self, id: i32) -> Result<Option<MissionInvite>>;
    async fn find_invites_by_user(&self, user_id: i32) -> Result<Vec<MissionInvite>>;
    async fn find_invites_details_by_user(&self, user_id: i32) -> Result<Vec<MissionInviteDetails>>;
    async fn find_invites_by_mission(&self, mission_id: i32) -> Result<Vec<MissionInvite>>;
    async fn update_status(&self, id: i32, status: String) -> Result<MissionInvite>;
    async fn check_exists(&self, mission_id: i32, user_id: i32) -> Result<bool>;
}
