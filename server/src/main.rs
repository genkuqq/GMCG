use enet::{Address, Enet, Event, Host};
use std::net::Ipv4Addr;
mod packets;
use packets::handle::handle_packet;
fn main() {
    let addr = Address::new(Ipv4Addr::new(127, 0, 0, 1), 1234);
    let enet = Enet::new().expect("Failed to initialize ENet");
    let mut host: Host<()> = enet
        .create_host(
            Some(&addr),
            32,
            enet::ChannelLimit::Maximum,
            enet::BandwidthLimit::Unlimited,
            enet::BandwidthLimit::Unlimited,
        )
        .expect("Failed to create host");

    println!("Server started {:?}", addr);
    loop {
        match host.service(1000).expect("service failed") {
            Some(Event::Connect(_)) => println!("new connection!"),
            Some(Event::Disconnect(..)) => {
                println!("disconnect!");
            }
            Some(Event::Receive {ref mut sender,ref packet,..}) =>{
                let packet_data = packet.data();
                match packet_data[0]{
                    1 =>{
                        handle_packet(packet_data,sender);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
