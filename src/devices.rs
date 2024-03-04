use crate::tw::EventMonitor;
use crate::tw::*;
use std::process::Command;

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
        get_usb_devices_physical().await;
    }
}

async fn get_usb_devices() {
    let mut result = String::new();
    let command_str = "cat /proc/bus/input/devices";
    if let Ok(res) = Command::new("sh").arg("-c").arg(command_str).output() {
        result = String::from_utf8(res.stdout.to_vec()).unwrap();
    }
    println!("{}", result)
}

async fn get_usb_devices_physical() {
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
    println!("{:#?}", devices)
}
