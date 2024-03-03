use crate::devices::*;

pub enum Monitors {
    USB(USBMon)
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

impl EventMonitor for Monitors {
    fn check(&self) {
        println!("check");
    }
}
