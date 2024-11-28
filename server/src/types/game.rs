use std::collections::HashMap;

use super::card::Card; 
use super::player::{Player, PlayerID};

#[derive(PartialEq)]
pub enum Stage{
    PreGame,
    InGame,
    Ended
}

pub struct GameState{
    pub players: HashMap<PlayerID, Player>,
    pub stage: Stage,
    pub deck: Vec<Card>,
    pub active_player: PlayerID,
}

pub enum GameEvent{
    BeginGame {first_player: PlayerID},
    EndGame {reason: String},
    PlayerJoined {player_id: PlayerID, table_pos: u8},
    PlayerDisconnected {player_id: PlayerID},
    PlayCard {player_id:PlayerID, card: Card}
}

