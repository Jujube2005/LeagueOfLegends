use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ChatMessage {
    pub mission_id: i32,
    pub user_id: Option<i32>,
    pub user_display_name: Option<String>,
    pub user_avatar_url: Option<String>,
    pub content: String,
    pub type_: String,
    pub created_at: String,
}

#[derive(Clone)]
pub struct MissionRealtimeService {
    // Map mission_id -> broadcast channel
    channels: Arc<Mutex<HashMap<i32, broadcast::Sender<ChatMessage>>>>,
}

impl MissionRealtimeService {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_channel(&self, mission_id: i32) -> broadcast::Sender<ChatMessage> {
        let mut channels = self.channels.lock().unwrap();
        
        if let Some(tx) = channels.get(&mission_id) {
            return tx.clone();
        }

        let (tx, _rx) = broadcast::channel(100);
        channels.insert(mission_id, tx.clone());
        
        tx
    }

    pub fn subscribe(&self, mission_id: i32) -> broadcast::Receiver<ChatMessage> {
        self.get_channel(mission_id).subscribe()
    }

    pub fn broadcast(&self, mission_id: i32, message: ChatMessage) {
        let tx = self.get_channel(mission_id);
        // We ignore error if there are no active receivers
        let _ = tx.send(message);
    }
}
