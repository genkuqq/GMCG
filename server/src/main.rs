use enet::*;
use pool::IdlePool;
use types::user::{self, User};
use std::net::Ipv4Addr;
use std::time::Duration;
use std::sync::Arc;

mod pool;
mod types;

#[tokio::main]
async fn main(){
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
    let pool = Arc::new(IdlePool::default());
    loop {
        if let Ok(Some(event)) = host.service(Duration::from_secs(1)) {
            match &event.kind {
                EventKind::Connect => {
                    println!("new connection!");
                    let user = User::new(event.peer_id);
                    pool.add_user_to_pool(user).await;
                    let pla = pool.players.lock().await;
                    for p in pla.iter(){
                        println!("ID: {:?}, State: {:?}", p.0, p.1);
                    }
                }
                EventKind::Disconnect { .. } => {
                    println!("disconnect!");
                    pool.remove_user_from_pool(event.peer_id).await;
                },
                EventKind::Receive {
                    channel_id,
                    packet,
                } => {
                    let data = packet.data();
                    match data[0]{
                        0 => {
                            print!("test");
                            pool.clone().set_user_state(event.peer_id, user::UserState::Searching);
                        },
                        1 => print!("data 1"),
                        _ => print!("baska")
                    }
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

