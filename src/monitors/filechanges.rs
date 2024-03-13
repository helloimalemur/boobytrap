use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::LockResult;
use Fasching::create_snapshot;
use Fasching::hasher::HashType;
use Fasching::snapshot::{FileMetadata, Snapshot};
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

        // println!("{:#?}", file_changes.snapshots);

        file_changes
    }
}



impl EventMonitor for FileChanges {
    async fn check(&mut self) {
        if self.step > 10 {
            println!("check fs changes: {}", self.triggered);

            match compare_snapshots(self, self.settings_map.clone()).await {
                None => {}
                Some(e) => {
                    match e.0 {
                        SnapshotChangeType::None => {}
                        SnapshotChangeType::Created => {
                            println!("File Created Alert!\n{:#?}", e.1);
                            let message = format!("File Creation Detected: {:?}", e.1.created);
                            fs_changes_alert(message, self.settings_map.clone()).await
                        }
                        SnapshotChangeType::Deleted => {
                            println!("File Deleted Alert!\n{:#?}", e.1);
                            let message = format!("File Deletion Detected: {:?}", e.1.deleted);
                            fs_changes_alert(message, self.settings_map.clone()).await
                        }
                        SnapshotChangeType::Changed => {
                            println!("File Change Alert!\n{:#?}", e.1);
                            let message = format!("File Change Detected: {:?}", e.1.changed);
                            fs_changes_alert(message, self.settings_map.clone()).await
                        }
                    }
                }
            }

            self.triggered = false;
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

enum SnapshotChangeType {
    None,
    Created,
    Deleted,
    Changed
}

#[derive(Debug)]
pub struct SnapshotCompareResult {
    created: Vec<String>,
    deleted: Vec<String>,
    changed: Vec<String>
}

async fn compare_snapshots(file_changes: &mut FileChanges, settings_map: HashMap<String, String>) -> Option<(SnapshotChangeType, SnapshotCompareResult)> {
    let mut success = true;
    let mut created: Vec<String> = vec![];
    let mut deleted: Vec<String> = vec![];
    let mut changed: Vec<String> = vec![];

    if let Some(last) = file_changes.snapshots.pop() {
        let current = create_snapshot(last.root_path.as_str(), last.hash_type);

        match last.file_hashes.lock() {
            Ok(mut last_lock) => {

                // for each entry in the hash list
                for last_entry in last_lock.iter() {

                    // check for deletion
                    if !Path::new(last_entry.0).exists() {
                        deleted.push(last_entry.0.to_string());
                        file_changes.triggered = true;
                    }

                    match current.file_hashes.lock() {
                        Ok(curr_lock) => {


                            match curr_lock.get(last_entry.0) {
                                Some(new_entry) => {

                                    // check for mis-matching checksum
                                    if !new_entry.check_sum.eq(&last_entry.1.check_sum) {
                                        file_changes.triggered = true;
                                        changed.push(new_entry.path.to_string());
                                    }

                                }
                                None => {success = false}
                            }

                        }
                        Err(_) => {success = false}

                    }

                }

            }
            Err(_) => {success = false}
        }

        match current.file_hashes.lock() {
            Ok(e) => {
                for new_entry in e.iter() {
                    // check for file creations
                    if last.file_hashes.lock().unwrap().get(new_entry.0).is_none() {
                        created.push(new_entry.0.to_string());
                        file_changes.triggered = true;
                    }
                }
            }
            Err(_) => {}
        }

        let mut return_type = SnapshotChangeType::None;
        if !created.is_empty() { return_type = SnapshotChangeType::Created; }
        if !deleted.is_empty() { return_type = SnapshotChangeType::Deleted; }
        if !changed.is_empty() { return_type = SnapshotChangeType::Changed; }

        file_changes.snapshots.push(current);
        println!("TOTAL SNAPSHOTS: {:#?}", file_changes.snapshots.len());
        Some((return_type, SnapshotCompareResult {
            created,
            deleted,
            changed,
        }))
    } else {
        None
    }
}


async fn fs_changes_alert(message: String, settings_map: HashMap<String, String>) {
    println!("{}", message);
    let _ = send_discord(message.as_str(), settings_map).await;
}
