use enet::{Packet, Peer};

pub fn handle_packet(packet: &[u8], sender: &mut Peer<()>){
    if packet.len() < 2{
        println!("Unknown Packet.");
        return;
    }
    let packet_type = packet[0];
    let variant_type = packet[1];
    let data = &packet[2..];
    let packet = Packet::new("test".as_bytes(), enet::PacketMode::ReliableSequenced).expect("Test");
    sender.send_packet(packet, 1);
}
