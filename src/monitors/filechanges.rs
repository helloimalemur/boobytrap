use std::collections::HashMap;
use Fasching::create_snapshot;
use Fasching::snapshot::Snapshot;
use crate::monitors::notify::send_discord;
use crate::tw::EventMonitor;
use crate::tw::*;


pub struct FileChanges {
    triggered: bool,
    monitored_directories: Vec<String>,
    current_snapshot: Vec<Snapshot>,
    settings_map: HashMap<String, String>
}

impl FileChanges {
    pub fn new(settings_map: HashMap<String, String>) -> Self {
        FileChanges {
            triggered: false,
            monitored_directories: vec![],
            current_snapshot: vec![],
            settings_map
        }
    }
}

impl EventMonitor for FileChanges {
    async fn check(&mut self) {
        if !compare_snapshots(self.settings_map.clone()) {
            self.triggered = true
        }
        if self.triggered {
            println!("File Change Alert!");
            fs_changes_alert(self.settings_map.clone()).await;
            let _ = send_discord("File Change Alert!!", self.settings_map.clone()).await;
        }
        println!("check fs changes: {}", self.triggered);
    }
}

async fn fs_changes_alert(settings_map: HashMap<String, String>) {
    // reboot_system(settings_map).await;
}

fn compare_snapshots(settings_map: HashMap<String, String>) -> bool {
    true
}
