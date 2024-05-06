use crate::monitors::actions::reboot_system;
use crate::tw::EventMonitor;
use config::Config;
use std::collections::HashMap;
use chrono::Local;

#[allow(unused)]
#[derive(Debug)]
pub struct NETMon {
    triggered: bool,
    interfaces: Vec<String>,
    settings_map: Config,
}

impl NETMon {
    pub fn new(settings_map: Config) -> Self {
        NETMon {
            triggered: false,
            interfaces: vec![],
            settings_map,
        }
    }
}

impl EventMonitor for NETMon {
    async fn check(&mut self) {
        if let Ok(check) = httping::ping("koonts.net", "", "https", 443).await {
            self.triggered = !check
        }
        if self.triggered {
            println!("{} :: ALERT NET", Local::now());
            net_alert(self.settings_map.clone()).await;
        }
        // println!("check net: {}", self.triggered);
    }
}

async fn net_alert(settings_map: Config) {
    reboot_system(settings_map).await;
}
