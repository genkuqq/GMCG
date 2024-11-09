pub enum PacketType{
    Unknown,
    Hello,
    MatchMaking,
    GameState,
    Gameplay,
    Chat,
}

impl PacketType{
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => PacketType::Hello,
            2 => PacketType::MatchMaking,
            3 => PacketType::GameState,
            4 => PacketType::Gameplay,
            5 => PacketType::Chat,
            _ => PacketType::Unknown,
        }
    }
}
