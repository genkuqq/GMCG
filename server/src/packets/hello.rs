use enet::{Packet, Peer};

pub fn handle_hello(packet: &[u8], sender: &mut Peer<()>){
    if packet.len() < 2{
        println!("Unknown Packet.");
        return;
    }
    let packet = Packet::new("test".as_bytes(), enet::PacketMode::ReliableSequenced).expect("Test");
    sender.send_packet(packet, 1);
}
