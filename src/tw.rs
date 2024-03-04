use crate::devices::*;
use crate::network::*;
use config::Config;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub enum Monitors {
    USBMon(USBMon),
    NetMon(NETMon),
}
pub struct AppState {
    pub mon_usb: bool,
    pub detection_triggered: bool,
    pub monitors: Arc<Mutex<Vec<Monitors>>>,
    pub settings_map: HashMap<String, String>,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::builder();
        let settings = config
            .add_source(config::File::with_name("config/Settings.toml"))
            .build()
            .unwrap();
        let settings_map = settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();

        let mut monitors: Vec<Monitors> = vec![];
        monitors.push(Monitors::USBMon(USBMon::new()));
        monitors.push(Monitors::NetMon(NETMon::new()));

        AppState {
            mon_usb: true,
            detection_triggered: false,
            monitors: Arc::new(Mutex::new(monitors)),
            settings_map,
        }
    }

    pub async fn run(&mut self) {
        loop {
            let mut binding = self.monitors.lock();
            let bind = binding.as_mut().unwrap();
            for i in bind.iter_mut() {
                match i {
                    Monitors::USBMon(e) => {
                        e.check().await;
                    }
                    Monitors::NetMon(e) => {
                        e.check().await;
                    }
                }
                thread::sleep(Duration::new(1, 0))
            }
        }
    }
}

pub trait EventMonitor {
    async fn check(&mut self);
}
