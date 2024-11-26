use enet::*;
use types::MatchMaking;
use std::net::Ipv4Addr;
use std::time::Duration;

mod types;
mod utils;

fn main(){
    let enet = Enet::new().expect("could not initialize ENet");

    let local_addr = Address::new(Ipv4Addr::LOCALHOST, 9001);

    let mut host = enet
        .create_host::<()>(
            Some(&local_addr),
            10,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited,
        )
        .expect("could not create host");

    loop {
        if let Ok(Some(event)) = host.service(Duration::from_secs(1)) {
            match &event.kind {
                EventKind::Connect => {
                    println!("new connection!");
                    let id = event.peer_id;
                    print!("{:#?}", id);
                }
                EventKind::Disconnect { .. } => println!("disconnect!"),
                EventKind::Receive {
                    channel_id,
                    packet,
                } => {
                    println!(
                        "got packet on channel {}, content: '{}'",
                        channel_id,
                        std::str::from_utf8(packet.data()).unwrap()
                    );
                    let peer = host.peer_mut(event.peer_id).unwrap();
                    let test_packet = Packet::new(b"Hello".to_vec(), PacketMode::ReliableSequenced).unwrap();
                    peer.send_packet(test_packet, 0);
                    let a = event.peer_id;
                    print!("{:#?}", a);
                }
            }
        }
    }   
}
