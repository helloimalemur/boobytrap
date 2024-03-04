use std::thread;
use std::time::Duration;
use crate::devices::*;
use crate::network::*;


pub enum Monitors {
    USBMon(USBMon),
    NetMon(NETMon)
}
pub struct AppState {
    pub mon_usb: bool,
    pub detection_triggered: bool,
    pub monitors: Vec<Monitors>
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            mon_usb: true,
            detection_triggered: false,
            monitors: vec![],
        }
    }

    pub async fn run(&self) {
        loop {
            for i in self.monitors.iter() {
                match i {
                    Monitors::USBMon(e) => {e.check().await;}
                    Monitors::NetMon(e) => { e.check().await;}
                }
                thread::sleep(Duration::new(0,100000))
            }
        }
    }

}

pub trait EventMonitor {
    async fn check(&self);
}
