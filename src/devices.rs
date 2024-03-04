use crate::tw::EventMonitor;
use crate::tw::*;

pub struct USBMon {
    triggered: bool,
    devices: Vec<String>,
    total_devices: i64,
}

impl USBMon {
    pub fn new() -> Self {
        USBMon {
            triggered: false,
            devices: vec![],
            total_devices: 0,
        }
    }
}

impl EventMonitor for USBMon {
    async fn check(&mut self) {
        println!("check usb: {}", self.triggered);
    }
}
