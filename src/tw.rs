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

    pub fn run(&self) {
        loop {
            for i in self.monitors.iter() {
                match i {
                    Monitors::USBMon(e) => { e.check() }
                    Monitors::NetMon(e) => {e.check()}
                }
            }
            thread::sleep(Duration::new(1,0))
        }
    }

}

pub trait EventMonitor {
    fn check(&self);
}
