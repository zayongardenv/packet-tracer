pub mod packet_handler {
    use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
    use pnet::datalink::{self, Channel::Ethernet, DataLinkReceiver};
    use std::sync::{Arc, Mutex};

    pub struct PacketTracer {
        output: Arc<Mutex<String>>,
    }

    impl PacketTracer {
        pub fn new(output: Arc<Mutex<String>>) -> Self {
            PacketTracer { output }
        }

        pub fn start_tracking(&self) {
            let interfaces = datalink::interfaces();
            let interface = interfaces.into_iter().find(|iface| iface.is_up() && !iface.is_loopback()).unwrap();
            let (_, mut rx) = datalink::channel(&interface, Default::default()).unwrap();

            loop {
                match rx.next() {
                    Ok(packet) => {
                        let eth_packet = EthernetPacket::new(packet).unwrap();
                        let eth_type = eth_packet.get_ethertype();
                        let mut output = self.output.lock().unwrap();
                        output.push_str(&format!("Packet: {:?}\n", eth_type));
                    }
                    Err(_) => {}
                }
            }
        }
    }
}