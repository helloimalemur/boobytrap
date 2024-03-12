use std::collections::HashMap;
use Fasching::create_snapshot;
use Fasching::hasher::HashType;
use Fasching::snapshot::Snapshot;
use crate::monitors::filechanges;
use crate::monitors::notify::send_discord;
use crate::tw::EventMonitor;
use crate::tw::*;


pub struct FileChanges {
    triggered: bool,
    step: u16,
    monitored_directories: Vec<String>,
    snapshots: Vec<Snapshot>,
    hash_type: HashType,
    settings_map: HashMap<String, String>
}

impl FileChanges {
    pub fn new(settings_map: HashMap<String, String>) -> Self {

        let mut file_changes = FileChanges {
            triggered: false,
            step: 0,
            monitored_directories: load_directories(settings_map.clone()),
            snapshots: vec![],
            hash_type: get_hash_type(settings_map.clone()),
            settings_map,
        };

        for dir in &file_changes.monitored_directories {
            file_changes.snapshots.push(create_snapshot(dir.as_str(), HashType::BLAKE3));
        }

        println!("{:#?}", file_changes.snapshots);

        file_changes
    }
}



impl EventMonitor for FileChanges {
    async fn check(&mut self) {
        if self.step > 10 {
            if !compare_snapshots(self.settings_map.clone()) {
                self.triggered = true
            }
            if self.triggered {
                println!("File Change Alert!");
                fs_changes_alert(self.settings_map.clone()).await;
                let _ = send_discord("File Change Alert!!", self.settings_map.clone()).await;
            }
            println!("check fs changes: {}", self.triggered);
            self.step = 0;
        } else {
            self.step += 1;
        }
    }
}

fn load_directories(settings_map: HashMap<String, String>) -> Vec<String> {
    let mut dirs: Vec<String> = vec![];
    for i in settings_map.iter() {
        if i.0.starts_with("fs_mon_dir") {
            dirs.push(i.1.to_string())
        }
    }
    dirs
}

fn get_hash_type(settings_map: HashMap<String, String>) -> HashType {
    match settings_map.get("fs_mon_hash_type").unwrap().as_str() {
        "blake3" => HashType::BLAKE3,
        "SHA3" => HashType::SHA3,
        "MD5" => HashType::MD5,
        _ => HashType::BLAKE3,
    }
}

async fn fs_changes_alert(settings_map: HashMap<String, String>) {
    // reboot_system(settings_map).await;
}

fn compare_snapshots(settings_map: HashMap<String, String>) -> bool {
    true
}
