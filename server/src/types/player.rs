use super::card::Card;

pub type PlayerID = u64;

pub struct Player{
    pub id: PlayerID,
    pub state: PlayerState,
    pub hand: Vec<Card>,
}

pub enum PlayerState{
    Idle,
    Searching,
    InGame(u32)
}
