pub enum Suit{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

pub struct Card{
    pub rank: u8,
    pub suit: Suit,
}
