use async_trait::async_trait;
use crate::domain::entities::notification::Notification;

#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn send(&self, notification: Notification) -> Result<(), anyhow::Error>;
}
 