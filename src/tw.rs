use std::sync::{Arc, Mutex};
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
    pub monitors: Arc<Mutex<Vec<Monitors>>>
}

impl AppState {
    pub fn new() -> Self {
        let mut monitors: Vec<Monitors> = vec![];

        monitors.push(Monitors::USBMon(USBMon::new()));
        monitors.push(Monitors::NetMon(NETMon::new()));

        AppState {
            mon_usb: true,
            detection_triggered: false,
            monitors: Arc::new(Mutex::new(monitors)),
        }
    }

    pub async fn run(&mut self) {
        loop {
            let mut binding = self.monitors.lock();
            let bind = binding.as_mut().unwrap();
            for i in bind.iter_mut() {
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
    async fn check(&mut self);
}
