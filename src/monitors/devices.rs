use crate::boobytrap::EventMonitor;
use crate::monitors::actions::reboot_system;
use crate::monitors::notify::send_discord;
use chrono::Local;
use config::Config;
use std::process::{exit, Command};

#[derive(Debug)]
pub struct USBMon {
    triggered: bool,
    devices: Vec<String>,
    total_devices: usize,
    last_check: usize,
    settings_map: Config,
}

impl USBMon {
    pub fn new(settings_map: Config) -> Self {
        USBMon {
            triggered: false,
            devices: vec![],
            total_devices: 0,
            last_check: 0,
            settings_map,
        }
    }
}

impl EventMonitor for USBMon {
    async fn check(&mut self) {
        let mut new_dev = String::new();
        let mut miss_dev = String::new();
        let mut new_devices: Vec<String> = vec![];
        let n = get_usb_devices_physical().await;
        n.iter().for_each(|r| new_devices.push(r.to_string()));
        let d = get_usb_devices().await;
        d.iter().for_each(|r| new_devices.push(r.to_string()));

        if self.last_check != 0 && self.last_check != new_devices.len() {
            self.total_devices = self.devices.len();


            for entry in new_devices.iter() {
                if !self.devices.contains(&entry) {
                    new_dev = entry.clone();
                }
            }

            for entry in self.devices.iter() {
                if !new_devices.contains(&entry) {
                    miss_dev = entry.clone();
                }
            }

            println!("{:#?}", &new_devices.len());
            println!("{:#?}", self.devices.len());
            println!("{:#?}", &new_dev);



            self.devices.clone_from(&new_devices);

            match self.last_check < new_devices.len() {
                true => {
                    self.triggered = true;
                    println!(
                        "{} :: Total USB devices INCREASED: {} :: {}",
                        Local::now(),
                        self.total_devices,
                        &new_dev
                    );
                }
                false => {
                    println!(
                        "{} :: Total USB devices DECREASED: {} :: {}",
                        Local::now(),
                        self.total_devices,
                        &new_dev
                    );
                }
            }

            self.last_check = self.total_devices;
        } else if self.last_check == 0 {
            self.devices.clone_from(&new_devices);
            self.total_devices = new_devices.len();
            self.last_check = self.total_devices;
            println!("Starting..");
            println!(
                "{} :: Total USB devices: {}",
                Local::now(),
                self.total_devices
            );
        }

        // println!(
        //     "check usb: {}, count: {}",
        //     self.triggered, self.total_devices
        // );

        if self.triggered {
            println!("ALERT USB");
            usb_triggered(self.settings_map.clone()).await;
            println!("{} :: USB count: {} :: {}", Local::now(), self.total_devices, &new_dev);
            self.triggered = false;
        }
    }
}

async fn get_usb_devices() -> Vec<String> {
    let mut devices: Vec<String> = vec![];
    #[allow(unused)]
    let mut result = String::new();
    // let command_str = "lsusb";
    let command_str = "lsusb | cut -d ' ' -f 7-16";
    if let Ok(res) = Command::new("sh").arg("-c").arg(command_str).output() {
        result = String::from_utf8(res.stdout.to_vec()).unwrap();
        result.split('\n').for_each(|r| {
            if !r.is_empty() {
                devices.push(r.to_string());
            }
        })
    }
    // println!("{}", result)
    devices
}

async fn get_usb_devices_physical() -> Vec<String> {
    let mut devices: Vec<String> = vec![];
    #[allow(unused)]
    let mut result = String::new();
    let command_str = "cat /proc/bus/input/devices | grep 'S:'";
    if let Ok(res) = Command::new("sh").arg("-c").arg(command_str).output() {
        result = String::from_utf8(res.stdout.to_vec()).unwrap();
        result.split('\n').for_each(|r| {
            if !r.is_empty() {
                devices.push(r.to_string());
            }
        })
    }
    // println!("{:#?}", devices);
    devices
}

async fn usb_triggered(settings_map: Config) {
    if settings_map
        .get::<String>("reboot_on_increase_of_usb_devices")
        .unwrap()
        .eq_ignore_ascii_case("true")
    {
        reboot_system(settings_map.clone()).await;
    }
    if settings_map
        .get::<String>("notify_on_increase_of_usb_devices")
        .unwrap()
        .eq_ignore_ascii_case("true")
    {
        let _ = send_discord("USB triggered", settings_map.clone()).await;
    }
}
