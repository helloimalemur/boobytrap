use crate::boobytrap::EventMonitor;
use crate::monitors::notify::send_discord;
use chrono::Local;
use config::Config;
use filesystem_hashing::hasher::HashType;
use filesystem_hashing::snapshot::Snapshot;
use filesystem_hashing::{compare_snapshots, create_snapshot};
use std::path::Path;
use std::{env, fs};
use std::sync::{Arc, Mutex};

#[allow(unused)]
#[derive(Debug)]
pub struct FileChanges {
    triggered: bool,
    step: u16,
    monitored_directories: Vec<String>,
    snapshots: Vec<Snapshot>,
    hash_type: HashType,
    settings_map: Config,
    black_list: Vec<String>,
    app_cache_path: String,
}

impl FileChanges {
    pub fn new(settings_map: Config, blacklist_file_path: String, app_cache_path: String) -> Self {
        let mut file_changes = FileChanges {
            triggered: false,
            step: 0,
            monitored_directories: load_directories(settings_map.clone()),
            snapshots: vec![],
            hash_type: get_hash_type(settings_map.clone()),
            settings_map,
            black_list: load_blacklist(blacklist_file_path),
            app_cache_path,
        };

        // load and push blacklisted directories
        println!("Blacklisted: {:?}", file_changes.black_list);

        file_changes.load_state();

        if file_changes.snapshots.is_empty() {
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
            let mut count = 0;
            println!("File Count:");
            file_changes.snapshots.iter().for_each(|s| {
                let s_len = s.file_hashes.lock().unwrap().len();
                println!("{} ---- {}", s_len, s.root_path);
                count += s_len
            });
            // println!("{:#?}", file_changes.snapshots);

            let message = format!(
                "{} :: Filesystem Snapshot Creation Successful\n\nTotal files: {}\n",
                Local::now(),
                count
            );
            println!("{}", message);
        }

        file_changes.save_state();
        file_changes
    }

    fn save_state(&mut self) {
        let snapshots_path = format!("{}snapshots/", self.app_cache_path);
        // println!("{}", snapshots_path);
        let _ = fs::create_dir_all(Path::new(snapshots_path.as_str()));
        self.snapshots.iter().for_each(|snapshot: &Snapshot| {
            let root_path_hash = blake3::hash(snapshot.root_path.as_bytes()).to_string();
            // println!("Exporting: {}", root_path_hash);
            let path = format!("{}/snapshots/{}", self.app_cache_path, root_path_hash);
            // println!("{:?}", snapshot);
            let sn = Snapshot {
                file_hashes: Arc::new(Mutex::new(snapshot.file_hashes.lock().unwrap().clone())),
                black_list: snapshot.black_list.clone(),
                root_path: snapshot.root_path.clone(),
                hash_type: HashType::BLAKE3,
                uuid: snapshot.uuid.clone(),
                date_created: snapshot.date_created.clone(),
            };

            if filesystem_hashing::export_snapshot(sn, path, true, false).is_err() {
                println!("WARNING: could not save state")
            }
        });
    }

    fn load_state(&mut self) {
        let snapshots_path = format!("{}snapshots/", self.app_cache_path);
        let mut snapshots: Vec<Snapshot> = vec![];
        let mut count = 0;
        if Path::new(snapshots_path.as_str()).exists() {
            let dir_vec = walkdir::WalkDir::new(snapshots_path)
                .contents_first(true)
                .into_iter()
                .map(|e| e.unwrap().path().to_str().unwrap().to_string())
                .filter(|a| {Path::new(a).is_file()})
                .collect::<Vec<String>>();
            // println!("{:?}", dir_vec);
            dir_vec.iter().for_each(|dir| {
                println!("{}", dir);
                let import = filesystem_hashing::import_snapshot(dir.to_string(), false).unwrap();
                count += import.file_hashes.lock().unwrap().len();
                snapshots.push(import)
            });

            self.snapshots.clone_from(&snapshots);
            println!("State Loaded..{} files", count);
            // drop(snapshots)
            // println!("{:?}", dir_vec);
        }
    }
}

impl EventMonitor for FileChanges {
    async fn check(&mut self) {
        match compare_all_snapshots(self, self.settings_map.clone(), self.black_list.clone()).await
        {
            None => {}
            Some(e) => match e.0 {
                SnapshotChangeType::None => {
                    // let message = format!("{} :: File System Unchanged",Local::now());
                    // println!("{}", message);
                    print!(".")
                }
                SnapshotChangeType::Created => {
                    // println!("{} :: File Created Alert!\n{:#?}", Local::now(), e.1);
                    let message = format!(
                        "{} :: File Creation Detected: {:?}",
                        Local::now(),
                        e.1.created
                    );
                    fs_changes_alert(message, self.settings_map.clone()).await
                }
                SnapshotChangeType::Deleted => {
                    // println!("{} :: File Deleted Alert!\n{:#?}", Local::now(), e.1);
                    let message = format!(
                        "{} :: File Deletion Detected: {:?}",
                        Local::now(),
                        e.1.deleted
                    );
                    fs_changes_alert(message, self.settings_map.clone()).await
                }
                SnapshotChangeType::Changed => {
                    // println!("{} :: File Change Alert!\n{:#?}", Local::now(), e.1);
                    let message = format!(
                        "{} :: File Change Detected: {:?}",
                        Local::now(),
                        e.1.changed
                    );
                    fs_changes_alert(message, self.settings_map.clone()).await
                }
            },
        }

        self.triggered = false;
        self.save_state()
    }
}

fn load_directories(settings_map: Config) -> Vec<String> {
    let mut dirs: Vec<String> = vec![];
    let mon_dirs = settings_map.get::<Vec<String>>("fs_mon_dir").unwrap();
    for i in mon_dirs.iter() {
        if i.contains('$') {
            let env_var = i.replace('$', "");
            let env_ret = env::var(env_var).unwrap();
            let split = env_ret.split(':').collect::<Vec<&str>>();
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

fn load_blacklist(blacklist_file_path: String) -> Vec<String> {
    let mut black_list: Vec<String> = vec![];
    if let Ok(file) = fs::read_to_string(Path::new(&blacklist_file_path)) {
        for line in file.lines() {
            if !line.is_empty() {
                black_list.push(line.to_string());
            }
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
        let x = load_blacklist("".to_string());
        println!("{:#?}", x);
    }
}
