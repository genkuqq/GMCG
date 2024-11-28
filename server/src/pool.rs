use std::collections::HashMap;
use enet::PeerID;
use tokio::sync::Mutex;
use crate::types::user::{User, UserState};

pub struct IdlePool{
    pub players: Mutex<HashMap<PeerID,User>>,
}

impl IdlePool {
    pub fn default() -> Self{
        Self{
            players: HashMap::new().into(),
        }
    }

    pub async fn add_user_to_pool(&self, user: User){
        let mut pool_lock = self.players.lock().await;
        pool_lock.insert(user.id, user);
    }

    pub async fn remove_user_from_pool(&self, user_id: PeerID){
        let mut pool_lock = self.players.lock().await;
        pool_lock.remove(&user_id);
    }

    pub async fn set_user_state(&self, user_id: PeerID, state: UserState) {
    let mut pool_lock = self.players.lock().await;
    
    if let Some(player) = pool_lock.get_mut(&user_id) {
        println!("User found: {:?}, changing state to {:?}", player.id, state);
        player.state = state;
    } else {
        println!("User with ID {:?} not found in the pool!", user_id);
    }
}
}


