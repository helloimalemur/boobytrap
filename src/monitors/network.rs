use std::collections::HashMap;
use crate::tw::EventMonitor;
use crate::tw::*;
use crate::monitors::actions::reboot_system;

pub struct NETMon {
    triggered: bool,
    interfaces: Vec<String>,
    settings_map: HashMap<String, String>
}

impl NETMon {
    pub fn new(settings_map: HashMap<String, String>) -> Self {
        NETMon {
            triggered: false,
            interfaces: vec![],
            settings_map
        }
    }
}

impl EventMonitor for NETMon {
    async fn check(&mut self) {
        if let Ok(check) = httping::ping("koonts.net", "", "https", 443).await {
            self.triggered = !check
        }
        if self.triggered {
            println!("ALERT NET");
            net_alert(self.settings_map.clone()).await;
        }
        println!("check net: {}", self.triggered);
    }
}

async fn net_alert(settings_map: HashMap<String, String>) {
    reboot_system(settings_map).await;
}
