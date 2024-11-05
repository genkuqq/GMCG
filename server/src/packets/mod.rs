pub mod hello;
use crate::packets::hello::handle_hello;

pub enum PacketType{
    Unknown,
    Hello,
}

impl PacketType{
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => PacketType::Hello,
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
    let data = &packet[2..];
    match PacketType::from_id(packet_type){
        PacketType::Hello => handle_hello(data),
        _ => (),
    }
}
