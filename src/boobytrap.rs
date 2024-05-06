use std::fs;
use std::path::Path;
use std::process::exit;
use crate::monitors::devices::USBMon;
use crate::monitors::filechanges::FileChanges;
use crate::monitors::network::NETMon;
use crate::monitors::ssh_burn_file::SSHBurnMon;
use config::Config;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub enum Monitors {
    USBMon(USBMon),
    NetMon(NETMon),
    SSHBurnMon(SSHBurnMon),
    FileChanges(FileChanges),
}
pub struct AppState {
    pub mon_usb: bool,
    pub detection_triggered: bool,
    pub monitors: Arc<Mutex<Vec<Monitors>>>,
    pub settings_map: Config,
}

impl AppState {
    pub fn new() -> Self {
        // let config = Config::builder();
        // let settings = config
        //     .add_source(config::File::with_name("config/Settings.toml"))
        //     .build()
        //     .unwrap();
        // // let settings_map = settings
        // //     .try_deserialize::<HashMap<String, String>>()
        // //     .unwrap();
        //
        // let mut monitors: Vec<Monitors> = vec![];
        //
        // if settings
        //     .get::<String>("usb_mon_enabled")
        //     .unwrap()
        //     .eq_ignore_ascii_case("true")
        // {
        //     monitors.push(Monitors::USBMon(USBMon::new(settings.clone())));
        // }
        // if settings
        //     .get::<String>("net_mon_enabled")
        //     .unwrap()
        //     .eq_ignore_ascii_case("true")
        // {
        //     monitors.push(Monitors::NetMon(NETMon::new(settings.clone())));
        // }
        // if settings
        //     .get::<String>("burn_file_mon_enabled")
        //     .unwrap()
        //     .eq_ignore_ascii_case("true")
        // {
        //     monitors.push(Monitors::SSHBurnMon(SSHBurnMon::new(settings.clone())));
        // }
        //
        // if settings
        //     .get::<String>("fs_mon_enabled")
        //     .unwrap()
        //     .eq_ignore_ascii_case("true")
        // {
        //     monitors.push(Monitors::FileChanges(FileChanges::new(settings.clone())));
        // }

        AppState {
            mon_usb: true,
            detection_triggered: false,
            monitors: Arc::new(Mutex::new(vec![])),
            settings_map: Config::default(),
        }
    }

    pub fn config(&mut self) {
        println!("Config..");
        // check for config file if it doesn't exist write default config

        let mut cache_dir = String::new();
        let cur_user = whoami::username();
        if cur_user.eq_ignore_ascii_case("root") {
            let _ = fs::create_dir_all(Path::new("/root/.cache/boobytrap/config/"));
            cache_dir = "/root/.cache/boobytrap/".to_string();
        } else {
            let create_dir = format!("/home/{}/.cache/boobytrap/config/", cur_user);
            let _ = fs::create_dir_all(Path::new(create_dir.as_str()));
            cache_dir = format!("/home/{}/.cache/boobytrap/", cur_user);
        }

        let settings_file_path = format!("{}config/Settings.toml", cache_dir);

        if !Path::new(settings_file_path.as_str()).exists() {
            println!("Settings.toml does not exist");
            exit(1)
        }

        let config = Config::builder();
        let settings = config
            .add_source(config::File::with_name(settings_file_path.as_str()))
            .build()
            .unwrap();


        let mut monitors: Vec<Monitors> = vec![];

        if settings
            .get::<String>("usb_mon_enabled")
            .unwrap()
            .eq_ignore_ascii_case("true")
        {
            monitors.push(Monitors::USBMon(USBMon::new(settings.clone())));
        }
        if settings
            .get::<String>("net_mon_enabled")
            .unwrap()
            .eq_ignore_ascii_case("true")
        {
            monitors.push(Monitors::NetMon(NETMon::new(settings.clone())));
        }
        if settings
            .get::<String>("burn_file_mon_enabled")
            .unwrap()
            .eq_ignore_ascii_case("true")
        {
            monitors.push(Monitors::SSHBurnMon(SSHBurnMon::new(settings.clone())));
        }

        if settings
            .get::<String>("fs_mon_enabled")
            .unwrap()
            .eq_ignore_ascii_case("true")
        {
            monitors.push(Monitors::FileChanges(FileChanges::new(settings.clone())));
        }

        self.monitors.clone_from(&Arc::new(Mutex::new(monitors)));
        self.settings_map.clone_from(&settings)
    }

    pub async fn run(&mut self) {
        let fs_check_tick = self
            .settings_map
            .get::<String>("fs_tick_delay_seconds")
            .expect("tick_delay_seconds not found in Settings.toml");
        let n_fs_check_tick = fs_check_tick
            .parse::<u64>()
            .expect("unable to parse fs_tick_delay_seconds");
        let tick = self
            .settings_map
            .get::<String>("tick_delay_seconds")
            .expect("tick_delay_seconds not found in Settings.toml");
        let n_tick = tick
            .parse::<u64>()
            .expect("unable to parse tick_delay_seconds");
        let mut last = SystemTime::now();
        loop {
            let mut binding = self.monitors.lock();
            let bind = binding.as_mut().unwrap();
            for i in bind.iter_mut() {
                match i {
                    Monitors::USBMon(e) => {
                        // println!("{:#?}", e);
                        e.check().await;
                    }
                    Monitors::NetMon(e) => {
                        // println!("{:#?}", e);
                        e.check().await;
                    }
                    Monitors::SSHBurnMon(e) => {
                        // println!("{:#?}", e);
                        e.check().await;
                    }
                    Monitors::FileChanges(e) => {
                        // println!("{:#?}", e);
                        let now = SystemTime::now();
                        let dur_since = now.duration_since(last).unwrap();
                        if dur_since.as_secs() > n_fs_check_tick {
                            // println!("fs_tick");
                            last = SystemTime::now();
                            e.check().await;
                        }
                    }
                }
                tokio::time::sleep(Duration::new(n_tick, 0)).await;
                // println!("tick");
            }
        }
    }
}

pub trait EventMonitor {
    async fn check(&mut self);
}