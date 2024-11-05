pub mod hello;
use crate::packets::hello::handle_hello;

pub enum PacketType{
    Hello = 1,
}

impl PacketType{
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(PacketType::Hello),
            _ => None,
        }
    }
}

pub fn handle_packet(packet: &[u8]){
    if packet.len() < 2{
        println!("Unknown Packet.");
        return;
    }
    let packet_type = packet[0];
    let data = packet[2..];
    match packet_type {
        1 => handle_hello(),
    }
}
