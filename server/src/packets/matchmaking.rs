use enet::{Packet, Peer};

use crate::matchmaking::{Matchmaking, Player};

pub fn handle_matchmaking(packet: &[u8], sender: &mut Peer<()>, matchmaking:&mut Matchmaking){
    if packet.len() < 2{
        println!("Unknown Packet.");
        return;
    }
    let variant_type = packet[1];
    let data = &packet[2..];
    match variant_type{
        // Join Queue
        1 => {
            let player_id = u32::from_le_bytes(data[2..].try_into().unwrap());
            let player = Player{
                id: player_id
        };
            matchmaking.add_player(player);
        }
        // Leave Queue
        2 => {
            let player_id = u32::from_le_bytes(data[2..].try_into().unwrap());
            let player = Player{
                id: player_id
            };
            matchmaking.remove_player(player);
        }

        _ => ()
    }
    let packet = Packet::new("test".as_bytes(), enet::PacketMode::ReliableSequenced).expect("Test");
    sender.send_packet(packet, 1);
}
