use crate::types::matchmaking::MatchmakingVariant;
struct DataPacket{
    player_id: u8,
}
impl DataPacket{
    fn from_bytes(data: &[u8]) -> Option<Self>{
        if data.len() <2{
            return None;
        }
        let player_id = data[0];
        Some(DataPacket{
            player_id
        }) 
    } 
} 
pub fn handle_matchmaking(variant:u8,data: &[u8]){ 
    match MatchmakingVariant::from_id(variant){ 
        MatchmakingVariant::StartSearch =>{
            DataPacket::from_bytes(data)
        } 
        MatchmakingVariant::CancelSearch =>{
        } 
        MatchmakingVariant::Unknown => {
            println!("Unknown packet.") 
        }
    }
}
