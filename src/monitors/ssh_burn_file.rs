use std::collections::HashMap;
use std::fmt::format;
use std::process::Command;
use chrono::{DateTime, Utc};
use crate::tw::EventMonitor;

pub struct SSHBurnMon {
    triggered: bool,
    settings_map: HashMap<String, String>,
    last_check: DateTime<Utc>,
}

impl SSHBurnMon {
    pub fn new(settings_map: HashMap<String, String>) -> Self {
        SSHBurnMon { triggered: false, settings_map, last_check: Utc::now() }
    }
}

impl EventMonitor for SSHBurnMon {
    async fn check(&mut self) {
        let ssh_check_burn_check_interval = self.settings_map.get("ssh_check_burn_check_interval").unwrap().parse::<i64>().unwrap();
        if Utc::now().signed_duration_since(self.last_check).num_seconds() > ssh_check_burn_check_interval {
            let ssh_check_burn_host = self.settings_map.get("ssh_check_burn_host").unwrap();
            let ssh_check_burn_user = self.settings_map.get("ssh_check_burn_user").unwrap();
            let ssh_check_burn_key = self.settings_map.get("ssh_check_burn_key").unwrap();
            let ssh_check_burn_path = self.settings_map.get("ssh_check_burn_path").unwrap();
            let addr = format!("{}@{}", ssh_check_burn_user, ssh_check_burn_host);
            let command_str = format!("if [ -f {} ]; then cat {}; fi", ssh_check_burn_path, ssh_check_burn_path);
            if let Ok(result) = Command::new("ssh")
                .arg("-i")
                .arg(ssh_check_burn_key)
                .arg(addr)
                .arg(command_str)
                .output() {
                println!("{}", String::from_utf8(result.stdout).unwrap().trim())
            }
            self.last_check = Utc::now();
        }
    }
}
