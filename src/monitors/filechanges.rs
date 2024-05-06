use crate::monitors::notify::send_discord;
use crate::tw::EventMonitor;
use config::Config;
use filesystem_hashing::hasher::HashType;
use filesystem_hashing::snapshot::Snapshot;
use filesystem_hashing::{compare_snapshots, create_snapshot};
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};
use chrono::Local;

#[allow(unused)]
pub struct FileChanges {
    triggered: bool,
    step: u16,
    monitored_directories: Vec<String>,
    snapshots: Vec<Snapshot>,
    hash_type: HashType,
    settings_map: Config,
    black_list: Vec<String>,
}

impl FileChanges {
    pub fn new(settings_map: Config) -> Self {
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
                if let Ok(snapshot) = create_snapshot(
                    dir.as_str(),
                    HashType::BLAKE3,
                    file_changes.black_list.clone(),
                    false,
                ) {
                    file_changes.snapshots.push(snapshot);
                }
            }
        }

        // println!("{:#?}", file_changes.snapshots);

        file_changes
    }
}

impl EventMonitor for FileChanges {
    async fn check(&mut self) {
        if self.step > 10 {
            // println!("check fs changes: {}", self.triggered);
            match compare_all_snapshots(self, self.settings_map.clone(), self.black_list.clone())
                .await
            {
                None => {}
                Some(e) => match e.0 {
                    SnapshotChangeType::None => {}
                    SnapshotChangeType::Created => {
                        // println!("{} :: File Created Alert!\n{:#?}", Local::now(), e.1);
                        let message = format!("{} :: File Creation Detected: {:?}",Local::now() , e.1.created);
                        fs_changes_alert(message, self.settings_map.clone()).await
                    }
                    SnapshotChangeType::Deleted => {
                        // println!("{} :: File Deleted Alert!\n{:#?}", Local::now(), e.1);
                        let message = format!("{} :: File Deletion Detected: {:?}",Local::now() , e.1.deleted);
                        fs_changes_alert(message, self.settings_map.clone()).await
                    }
                    SnapshotChangeType::Changed => {
                        // println!("{} :: File Change Alert!\n{:#?}", Local::now(), e.1);
                        let message = format!("{} :: File Change Detected: {:?}",Local::now() , e.1.changed);
                        fs_changes_alert(message, self.settings_map.clone()).await
                    }
                },
            }

            self.triggered = false;
            self.step = 0;
        } else {
            self.step += 1;
        }
    }
}

fn load_directories(settings_map: Config) -> Vec<String> {
    let mut dirs: Vec<String> = vec![];
    let mon_dirs = settings_map.get::<Vec<String>>("fs_mon_dir").unwrap();
    for i in mon_dirs.iter() {
        if i.contains('$') {
            let env_var = i.replace("$", "");
            let env_ret = env::var(env_var).unwrap();
            let split = env_ret.split(":").collect::<Vec<&str>>();
            split.iter().for_each(|e| dirs.push(e.to_string()))
        } else {
            dirs.push(i.to_string())
        }
    }
    println!("Monitoring Directories: {:#?}", dirs);
    dirs
}

fn get_hash_type(settings_map: Config) -> HashType {
    match settings_map
        .get::<String>("fs_mon_hash_type")
        .unwrap()
        .as_str()
    {
        "blake3" => HashType::BLAKE3,
        "SHA3" => HashType::SHA3,
        "MD5" => HashType::MD5,
        _ => HashType::BLAKE3,
    }
}

fn load_blacklist() -> Vec<String> {
    let mut black_list: Vec<String> = vec![];

    if let Ok(file) = fs::read_to_string(Path::new("config/file_mon_blacklist")) {
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
    Changed,
}

#[derive(Debug)]
pub struct SnapshotCompareResult {
    pub created: Vec<String>,
    pub deleted: Vec<String>,
    pub changed: Vec<String>,
}

async fn compare_all_snapshots(
    file_changes: &mut FileChanges,
    _settings_map: Config,
    black_list: Vec<String>,
) -> Option<(SnapshotChangeType, SnapshotCompareResult)> {
    let mut created: Vec<String> = vec![];
    let mut deleted: Vec<String> = vec![];
    let mut changed: Vec<String> = vec![];

    let mut to_remove: Vec<usize> = vec![];
    let mut new_sn: Vec<Snapshot> = vec![];

    for (ind, i) in file_changes.snapshots.iter().enumerate() {
        // println!("{:#?}", black_list);
        if let Ok(rehash) =
            Snapshot::new(i.root_path.as_ref(), i.hash_type, black_list.clone(), false)
        {
            if let Some(res) = compare_snapshots(i.clone(), rehash.clone(), false) {
                // println!("{}", i.root_path);
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
    }

    file_changes.snapshots.clear();
    file_changes.snapshots = new_sn;

    let mut return_type = SnapshotChangeType::None;
    if !created.is_empty() {
        return_type = SnapshotChangeType::Created;
    }
    if !deleted.is_empty() {
        return_type = SnapshotChangeType::Deleted;
    }
    if !changed.is_empty() {
        return_type = SnapshotChangeType::Changed;
    }

    // if !created.is_empty() || !deleted.is_empty() || !changed.is_empty() {
    //     println!("created: {:?}", created);
    //     println!("deleted: {:?}", deleted);
    //     println!("changed: {:?}", changed);
    // }

    Some((
        return_type,
        SnapshotCompareResult {
            created,
            deleted,
            changed,
        },
    ))
}

async fn fs_changes_alert(message: String, settings_map: Config) {
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
