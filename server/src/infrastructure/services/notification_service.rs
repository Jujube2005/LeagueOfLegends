// *เพิ่ม

use tokio::sync::broadcast;
use crate::domain::{services::notification_service::NotificationService, entities::notification::Notification};
use async_trait::async_trait;

#[derive(Clone)]
pub struct NotificationServiceImpl {
    sender: broadcast::Sender<Notification>,
}

impl NotificationServiceImpl {
    pub fn new(sender: broadcast::Sender<Notification>) -> Self {
        Self { sender }
    }
}

#[async_trait]
impl NotificationService for NotificationServiceImpl {
    async fn send(&self, notification: Notification) -> Result<(), anyhow::Error> {
        // We ignore the error if there are no receivers (it returns SendError, which is fine)
        let _ = self.sender.send(notification);
        Ok(())
    }
}
