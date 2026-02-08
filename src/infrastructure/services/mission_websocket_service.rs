use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};

#[derive(Clone)]
pub struct MissionWebSocketService {
    // Map mission_id -> broadcast channel
    rooms: Arc<RwLock<HashMap<i32, broadcast::Sender<String>>>>,
}

impl MissionWebSocketService {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_create_room(&self, mission_id: i32) -> broadcast::Sender<String> {
        let mut rooms = self.rooms.write().await;
        if let Some(tx) = rooms.get(&mission_id) {
            return tx.clone();
        }

        let (tx, _rx) = broadcast::channel(100);
        rooms.insert(mission_id, tx.clone());
        tx
    }

    pub async fn broadcast(&self, mission_id: i32, message: String) {
        let rooms = self.rooms.read().await;
        if let Some(tx) = rooms.get(&mission_id) {
            let _ = tx.send(message);
        }
    }
}
