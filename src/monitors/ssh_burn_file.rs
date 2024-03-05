use std::collections::HashMap;
use crate::tw::EventMonitor;

pub struct SSHBurnMon {
    triggered: bool,
    settings_map: HashMap<String, String>
}

impl SSHBurnMon {
    pub fn new(settings_map: HashMap<String, String>) -> Self {
        SSHBurnMon { triggered: false, settings_map }
    }
}

impl EventMonitor for SSHBurnMon {
    async fn check(&mut self) {
        let ssh_check_burn_host = self.settings_map.get("ssh_check_burn_host").unwrap();
        let ssh_check_burn_user = self.settings_map.get("ssh_check_burn_user").unwrap();
        let ssh_check_burn_key = self.settings_map.get("ssh_check_burn_key").unwrap();
        let ssh_check_burn_path = self.settings_map.get("ssh_check_burn_path").unwrap();


    }
}
