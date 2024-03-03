use crate::tw::EventMonitor;
use crate::tw::*;

pub struct USBMon {
    triggered: bool,
    devices: Vec<String>
}

impl USBMon {
    pub fn new() -> Self {
        USBMon { triggered: false, devices: vec![] }
    }

}

impl EventMonitor for USBMon {
    fn check(&self) {
        println!("check usb");
    }
}
