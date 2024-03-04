use trait_enum::trait_enum;
#[macro_use]
use crate::devices::*;



trait_enum!{
    pub enum Monitors: EventMonitor {
        USBMon
    }
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
        for i in self.monitors.iter() {
            i.check();
        }
    }

}

pub trait EventMonitor {
    fn check(&self);
}
