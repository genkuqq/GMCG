use std::net::UdpSocket;
use std::time::{Duration,SystemTime,Instant};
use renet::{ConnectionConfig,DefaultChannel,RenetServer,ServerEvent};
use renet::transport::{NetcodeServerTransport,ServerAuthentication,ServerConfig};

mod types;


fn main(){
    let mut server = RenetServer::new(ConnectionConfig::default());
    let socket: UdpSocket = UdpSocket::bind("127.0.0.1:1234").unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![socket.local_addr().unwrap()],
        authentication: ServerAuthentication::Unsecure
    };
    let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let mut last_updated = Instant::now();

    loop {
        let now = Instant::now();
        let delay = now - last_updated;
        server.update(delay);
        transport.update(delay, &mut server);
        last_updated = now;
        
        while let Some(event) = server.get_event() {
            match event {           
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client {client_id} connected");
                    server.send_message(client_id, 0, "Welcome!");
                    server.broadcast_message(DefaultChannel::ReliableOrdered, "A user joined to server.");

                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {client_id} disconnected: {reason}");
                }
            }
        }

        for client_id in server.clients_id() {
            while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
                println!("Message: {} {:?}",client_id,message);
            }
        }

        transport.send_packets(&mut server);
        std::thread::sleep(Duration::from_millis(16));

    }
}
