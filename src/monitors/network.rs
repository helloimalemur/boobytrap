use std::sync::Arc;
use crate::boobytrap::EventMonitor;
use crate::monitors::actions::reboot_system;
use chrono::Local;
use config::Config;
use netstat::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, SocketInfo};
use tokio::sync::Mutex;

#[allow(unused)]
#[derive(Debug)]
pub struct NETMon {
    triggered: bool,
    interfaces: Vec<String>,
    settings_map: Config,
    state: Arc<Mutex<Vec<SocketInfo>>>
}

impl NETMon {
    pub fn new(settings_map: Config) -> Self {

        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
        let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();
        let moved_sockets = sockets_info.clone();
        for si in moved_sockets {
            match si.protocol_socket_info {
                ProtocolSocketInfo::Tcp(tcp_si) => println!(
                    "TCP {}:{} -> {}:{} {:?} - {}",
                    tcp_si.local_addr,
                    tcp_si.local_port,
                    tcp_si.remote_addr,
                    tcp_si.remote_port,
                    si.associated_pids,
                    tcp_si.state
                ),
                ProtocolSocketInfo::Udp(udp_si) => println!(
                    "UDP {}:{} -> *:* {:?}",
                    udp_si.local_addr, udp_si.local_port, si.associated_pids
                ),
            }
        }

        NETMon {
            triggered: false,
            interfaces: vec![],
            settings_map,
            state: Arc::new(Mutex::new(sockets_info.clone()))
        }
    }
}

impl EventMonitor for NETMon {
    async fn check(&mut self) {
        if let Ok(check) = httping::ping("koonts.net", "", "https", 443).await {
            self.triggered = !check
        }
        if self.triggered {
            println!("{} :: ALERT NET", Local::now());
            net_alert(self.settings_map.clone()).await;
        }
        // println!("check net: {}", self.triggered);
    }
}

async fn net_alert(settings_map: Config) {
    reboot_system(settings_map).await;
}
