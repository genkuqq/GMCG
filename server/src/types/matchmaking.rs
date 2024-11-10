pub enum MatchmakingVariant{
    StartSearch,
    CancelSearch,
    Unknown,
}
impl MatchmakingVariant {
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => MatchmakingVariant::StartSearch,
            2 => MatchmakingVariant::CancelSearch,
            _ => MatchmakingVariant::Unknown,
        }
    }
}
pub struct MMPacket{
    player_id: u8,
}
