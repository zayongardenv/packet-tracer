use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, TextView};
use pnet::datalink::{self, Channel::Ethernet, DataLinkReceiver};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::Packet;
use std::sync::{Arc, Mutex};
use std::thread;

struct AppState {
    output: Arc<Mutex<String>>,
}

fn main() {
    let application = Application::new(Some("com.example.packet_tracer"), Default::default());
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Packet Tracer");
        window.set_default_size(800, 600);

        let output = Arc::new(Mutex::new(String::new()));
        let output_clone = Arc::clone(&output);

        let text_view = TextView::new();
        text_view.set_editable(false);
        let scrollable = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrollable.add(&text_view);
        window.add(&scrollable);

        let button = Button::with_label("Start Tracking");
        window.add(&button);
        button.connect_clicked(move |_| {
            let output_clone = Arc::clone(&output_clone);
            thread::spawn(move || {
                let interfaces = datalink::interfaces();
                let interface = interfaces.into_iter().find(|iface| iface.is_up() && !iface.is_loopback()).unwrap();
                let (_, mut rx) = datalink::channel(&interface, Default::default()).unwrap();

                loop {
                    match rx.next() {
                        Ok(packet) => {
                            let eth_packet = EthernetPacket::new(packet).unwrap();
                            let eth_type = eth_packet.get_ethertype();
                            let mut output = output_clone.lock().unwrap();
                            output.push_str(&format!("Packet: {:?}\n", eth_type));
                        }
                        Err(_) => {}
                    }
                }
            });
        });

        window.show_all();
    });

    application.run();
}