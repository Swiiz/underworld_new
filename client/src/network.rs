use std::net::ToSocketAddrs;

use common::network::{proto::network_protocol, AnyPacket, Protocol};
use serde::Serialize;
use uflow::{client::Client, SendMode};

pub struct NetworkClient {
    protocol: Protocol,
    socket: Client,
}

impl NetworkClient {
    pub fn connect_to(address: impl ToSocketAddrs) -> Self {
        Self {
            socket: Client::connect(address, Default::default()).expect("Invalid address"),
            protocol: network_protocol(),
        }
    }

    pub fn handle_packets(&mut self, mut handler: impl FnMut(AnyPacket)) {
        for event in self.socket.step() {
            match event {
                uflow::client::Event::Connect => {
                    println!("Connected to the server");
                    // TODO: Handle connection
                }
                uflow::client::Event::Disconnect => {
                    println!("Disconnected from the server, you may close the window");
                    // TODO: Handle disconnection
                }
                uflow::client::Event::Error(error) => {
                    panic!("Fatal network error: {error:?}");
                }
                uflow::client::Event::Receive(packet_data) => {
                    if let Some(p) = self.protocol.decode(&packet_data) {
                        handler(p);
                    }
                }
            }
        }
    }

    pub fn send<T: Serialize + 'static>(&mut self, packet: &T, mode: SendMode) {
        self.socket.send(self.protocol.encode(packet), 0, mode);
    }

    pub fn flush(&mut self) {
        self.socket.flush();
    }

    pub fn exit(&mut self) {
        self.socket.disconnect_now();
        let _ = self.socket.step();
    }
}
