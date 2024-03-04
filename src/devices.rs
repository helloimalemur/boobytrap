use crate::tw::EventMonitor;
use crate::tw::*;
use std::process::Command;

pub struct USBMon {
    triggered: bool,
    devices: Vec<String>,
    total_devices: usize,
    last_check: usize,
}

impl USBMon {
    pub fn new() -> Self {
        USBMon {
            triggered: false,
            devices: vec![],
            total_devices: 0,
            last_check: 0,
        }
    }
}

impl EventMonitor for USBMon {
    async fn check(&mut self) {
        let new_devices = get_usb_devices_physical().await;
        if self.last_check != 0 && self.last_check != new_devices.len() {
            if self.last_check < new_devices.len() {
                self.triggered = true;
                println!("Total devices: {}", self.total_devices);
            }
            self.devices = new_devices;
            self.total_devices = self.devices.len();
            self.last_check = self.total_devices;
        } else if self.last_check == 0 {
            self.devices = new_devices.clone();
            self.total_devices = new_devices.len();
            self.last_check = self.total_devices;
            println!("Total devices: {}", self.total_devices);
        }

        println!("check usb: {}", self.triggered);

        if self.triggered {
            println!("ALERT USB");
            self.triggered = false;
        }
    }
}

async fn get_usb_devices() {
    let mut result = String::new();
    let command_str = "cat /proc/bus/input/devices";
    if let Ok(res) = Command::new("sh").arg("-c").arg(command_str).output() {
        result = String::from_utf8(res.stdout.to_vec()).unwrap();
    }
    // println!("{}", result)
}

async fn get_usb_devices_physical() -> Vec<String> {
    let mut devices: Vec<String> = vec![];
    let mut result = String::new();
    let command_str = "cat /proc/bus/input/devices | grep 'S:'";
    if let Ok(res) = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()
    {
        result = String::from_utf8(res.stdout.to_vec()).unwrap();
        result.split('\n').for_each(|r| {
            if !r.split(' ').last().unwrap().to_string().is_empty() {
                devices.push(r.split(' ').last().unwrap().to_string())
            }
        })
    }
    // println!("{:#?}", devices);
    devices
}
