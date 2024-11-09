pub mod hello;
pub mod chat;
pub mod matchmaking;
pub mod gamestate;
pub mod gameplay;
use crate::packets::hello::handle_hello;
use crate::packets::chat::handle_chat;
use crate::packets::matchmaking::handle_matchmaking;
use crate::packets::gamestate::handle_gamestate;
use crate::packets::gameplay::handle_gameplay;

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

pub fn handle_packet(packet: &[u8]){
    if packet.len() < 2{
        println!("Unknown Packet.");
        return;
    }
    let packet_type = packet[0];
    let variant_type = packet[1];
    let data = &packet[2..];
    match PacketType::from_id(packet_type){
        PacketType::Hello => handle_hello(data),
        PacketType::MatchMaking => handle_matchmaking(variant_type,data),
        PacketType::GameState => handle_gamestate(variant_type,data),
        PacketType::Gameplay => handle_gameplay(variant_type,data),
        PacketType::Chat => handle_chat(variant_type, data),
        _ => (),
    }
}
