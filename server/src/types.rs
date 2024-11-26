use std::collections::HashMap;

type PlayerID = u64;
enum Suit{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

struct Card{
    rank: u8,
    suit: Suit,
}

struct Player{
    id: PlayerID,
    hand: Vec<Card>,
}

struct Game{
    players: HashMap<PlayerID, Player>,
}
