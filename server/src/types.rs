use std::collections::HashMap;

pub type PlayerID = u64;

pub enum Suit{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

pub struct Card{
    rank: u8,
    suit: Suit,
}

pub struct Player{
    id: PlayerID,
    hand: Vec<Card>,
}

pub enum Stage{
    PreGame,
    InGame,
    Ended
}

pub struct GameState{
    players: HashMap<PlayerID, Player>,
    stage: Stage,
    deck: Vec<Card>,
    active_player: PlayerID,
}

pub enum GameEvent{
    BeginGame {first_player: PlayerID},
    EndGame {reason: String},
    PlayerJoined {player_id: PlayerID, table_pos: u8},
    PlayerDisconnected {player_id: PlayerID},
    PlayCard {player_id:PlayerID, card: Card}
}
