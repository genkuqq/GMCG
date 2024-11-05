use enet::{Address, Enet, Event, Host};
use std::net::Ipv4Addr;

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
            Some(Event::Receive {
                channel_id,
                ref packet,
                ..
            }) => println!(
                "got packet on channel {}, content: '{}'",
                channel_id,
                std::str::from_utf8(packet.data()).unwrap()
            ),
            _ => (),
        }
    }
}
