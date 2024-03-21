use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, Read};
use std::path::Path;
use std::sync::LockResult;
use config::File;
use Fasching::{compare_snapshots, create_snapshot};
use Fasching::hasher::HashType;
use Fasching::snapshot::{FileMetadata, Snapshot};
use bytes::BytesMut;
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
    settings_map: HashMap<String, String>,
    black_list: Vec<String>
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
            black_list: load_blacklist(),
        };

        // load and push blacklisted directories
        println!("Blacklisted: {:?}", file_changes.black_list);

        for dir in &file_changes.monitored_directories {
            if !file_changes.black_list.contains(dir) {
                file_changes.snapshots.push(create_snapshot(dir.as_str(), HashType::BLAKE3, file_changes.black_list.clone()));
            }
        }

        // println!("{:#?}", file_changes.snapshots);

        file_changes
    }
}



impl EventMonitor for FileChanges {
    async fn check(&mut self) {
        if self.step > 10 {
            println!("check fs changes: {}", self.triggered);
            match compare_all_snapshots(self, self.settings_map.clone(), self.black_list.clone()).await {
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

fn load_blacklist() -> Vec<String> {
    let mut black_list: Vec<String> = vec![];

    if let Ok(mut file) = fs::read_to_string(Path::new("config/file_mon_blacklist")) {
        for line in file.lines() {
            black_list.push(line.to_string());
        }
    }

    black_list
}

enum SnapshotChangeType {
    None,
    Created,
    Deleted,
    Changed
}

#[derive(Debug)]
pub struct SnapshotCompareResult {
    pub created: Vec<String>,
    pub deleted: Vec<String>,
    pub changed: Vec<String>
}

async fn compare_all_snapshots(file_changes: &mut FileChanges, settings_map: HashMap<String, String>, black_list: Vec<String>) -> Option<(SnapshotChangeType, SnapshotCompareResult)> {
    let mut created: Vec<String> = vec![];
    let mut deleted: Vec<String> = vec![];
    let mut changed: Vec<String> = vec![];

    let mut to_remove: Vec<usize> = vec![];
    let mut new_sn: Vec<Snapshot> = vec![];

    for (ind, i) in file_changes.snapshots.iter().enumerate() {
        // println!("{:#?}", black_list);
        let rehash = Snapshot::new(i.root_path.as_ref(), i.hash_type, black_list.clone());

        if let Some(res) = compare_snapshots(i.clone(), rehash.clone()) {
            println!("{}", i.root_path);
            for c in res.1.created {
                created.push(c)
            }
            for d in res.1.deleted {
                deleted.push(d)
            }
            for ch in res.1.changed {
                changed.push(ch)
            }

            to_remove.push(ind);
            new_sn.push(rehash.clone());
        }
    }

    file_changes.snapshots.clear();
    file_changes.snapshots = new_sn;

    let mut return_type = SnapshotChangeType::None;
    if !created.is_empty() { return_type = SnapshotChangeType::Created; }
    if !deleted.is_empty() { return_type = SnapshotChangeType::Deleted; }
    if !changed.is_empty() { return_type = SnapshotChangeType::Changed; }

    println!("created: {:?}", created);
    println!("deleted: {:?}", deleted);
    println!("changed: {:?}", changed);


    Some((return_type, SnapshotCompareResult {
        created,
        deleted,
        changed,
    }))
}


async fn fs_changes_alert(message: String, settings_map: HashMap<String, String>) {
    println!("{}", message);
    let _ = send_discord(message.as_str(), settings_map).await;
}


#[cfg(test)]
mod tests {
    use crate::monitors::filechanges::load_blacklist;

    #[test]
    fn test_load_blacklist() {
        let x = load_blacklist();
        println!("{:#?}", x);
    }
}
