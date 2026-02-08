// *เพิ่ม

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub enum NotificationType {
    JoinMission,
    LeaveMission,
    MissionStatusUpdate,
}

#[derive(Debug, Clone, Serialize)]
pub struct Notification {
    pub recipient_id: Option<i32>, // None for broadcast, Some for specific user
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub metadata: Value,
}
