extern crate enet;

use enet::*;
use std::net::Ipv4Addr;
use std::time::Duration;

fn main() -> Result<(), Error> {
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
        // Wait 1 second for any events.
        match host.service(Duration::from_secs(1)) {
            Ok(Some(event)) => match &event.kind {
                &EventKind::Connect => {
                    println!("new connection!");
                    let id = event.peer_id;
                    print!("{:#?}",id);
                },
                &EventKind::Disconnect { .. } => println!("disconnect!"),
                &EventKind::Receive {
                    ref channel_id,
                    ref packet,
                } => {
                    println!(
                    "got packet on channel {}, content: '{}'",
                    channel_id,
                    std::str::from_utf8(packet.data()).unwrap());
                    let a = event.peer_id;
                    print!("{:#?}", a)
                },
            },
            Ok(None) => {
                // No event occurred within the timeout duration
            }
            Err(e) => {
                // Handle the error from `service`
                eprintln!("Error servicing host: {:?}", e);
            }
        }
    }
}
