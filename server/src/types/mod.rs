use std::collections::HashMap;

pub enum Card {
    SA,S1,S2,S3,S4,S5,S6,S7,S8,S9,S10,SJ,SQ,SK,
    HA,H1,H2,H3,H4,H5,H6,H7,H8,H9,H10,HJ,HQ,HK,
    DA,D1,D2,D3,D4,D5,D6,D7,D8,D9,D10,DJ,DQ,DK,
    CA,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,CJ,CQ,CK 
}

pub struct Player{
    pub name: String,
    pub cards: Vec<Card>,
}

pub enum GameStage{
    PreGame,
    InGame,
    Ended
}

type PlayerID = u64;

pub struct GameState{
    pub stage: GameStage,
    pub desk: Vec<Card>,
    pub active_player_id: PlayerID,
    pub players: HashMap<PlayerID,Player>,
    pub history: Vec<GameEvent>
}

impl Default for GameState{
    fn default() -> Self {
        Self{
            stage: GameStage::PreGame,
            desk: Vec::new(),
            active_player_id: 0,
            players: HashMap::new(),
            history: Vec::new()
        }
    }
}

pub enum EndGameReason{
    PlayerLeft {player_id: PlayerID},
    PlayerWon {winner: PlayerID}
}

pub enum GameEvent{
    BeginGame {first_player: PlayerID},
    EndGame {reason: EndGameReason},
    PlayerJoined {player_id: PlayerID, name: String},
    PlayerDisconnected {player_id: PlayerID},
    PlayCard {player_id: PlayerID, card: Card}
}
impl GameState{ 
    pub fn packet(&self, event: &GameEvent)
    { use GameEvent::*; 
        match event{ 
            BeginGame { first_player } => { 
                let player_is_unknown = self.players.contains_key(first_player); 
                if self.stage != GameStage::PreGame || player_is_unknown{ 
                    return false; 
                } 
                self.active_player_id = *first_player; 
                self.stage = GameStage::InGame; 
            }, 
            EndGame { reason } => { 
                match reason { 
                    EndGameReason::PlayerWon { winner: _ } => { 
                        if self.stage != GameStage::InGame { 
                            return false; 
                        } 
                    } 
                    _ => () 
                }
                self.stage = GameStage::Ended; 
            }, 
            PlayerJoined { player_id, name } => { 
                if self.players.contains_key(player_id){ 
                    return false; 
                } 
                self.players.insert(*player_id, Player{ 
                    name: name.to_string(), 
                    cards: vec![Card::SA,Card::HA]
                });
            },
            PlayerDisconnected { player_id } => {
                if !self.players.contains_key(player_id) {
                    return false;
                }
                self.players.remove(player_id);
            }
            PlayCard { player_id, card } => {
                if !self.players.contains_key(player_id) {
                    return false;
                }

                if self.active_player_id != *player_id {
                    return false;
                }
                if self.desk.len() > 52 {
                    return false;
                }
                self.desk.insert(self.desk.len()-1, card);
                self.active_player_id = self
                    .players
                    .keys()
                    .find(|id| *id != player_id)
                    .unwrap()
                    .clone();
            }
        }
    }
}
