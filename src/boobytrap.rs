use crate::default_config::{get_cache_dir, setup_service, write_default_blacklist, write_default_config};
use crate::monitors::devices::USBMon;
use crate::monitors::filechanges::FileChanges;
// use crate::monitors::network::NETMon;
use crate::monitors::ssh_burn_file::SSHBurnMon;
use config::Config;
use std::path::Path;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

pub trait EventMonitor {
    async fn check(&mut self);
}

#[derive(Debug)]
pub enum Monitors {
    USBMon(USBMon),
    // NetMon(NETMon),
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
        AppState {
            mon_usb: true,
            detection_triggered: false,
            monitors: Arc::new(Mutex::new(vec![])),
            settings_map: Config::default(),
        }
    }

    pub fn config(&mut self, args: Vec<String>) {
        let mut webhook = String::new();
        for arg in args {
            if arg.contains("install-service") {
                setup_service();
            }
            if arg.contains("webhook") {
                webhook = arg.split("=").last().unwrap().to_string();
            }
        }

        // check for config file if it doesn't exist write default config
        println!("Configuring..");
        let cache_dir = get_cache_dir();
        let settings_file_path = format!("{}config/Settings.toml", cache_dir);
        // sed -i 's,discord_webhook_url = "https://discord.com/api/webhooks/",discord_webhook_url = "$WEBHOOKURL",g'
        let sed_command = format!("'s,discord_webhook_url = \"https://discord.com/api/webhooks/\",discord_webhook_url = \"{}\",g'", webhook);
        if let Ok(o) = process::Command::new("sed")
            .arg("-i")
            .arg(sed_command.clone())
            .arg(settings_file_path.clone())
            .output() {
            println!("{}", String::from_utf8_lossy(&o.stdout));
        }


        if !Path::new(settings_file_path.as_str()).exists() {
            println!("Settings.toml does not exist");
            write_default_config(settings_file_path.clone());
        }
        let blacklist_file_path = format!("{}config/file_mon_blacklist", cache_dir);
        if !Path::new(blacklist_file_path.as_str()).exists() {
            println!("file_mon_blacklist does not exist");
            write_default_blacklist(blacklist_file_path.clone());
        }
        let config = Config::builder();
        let settings = config
            .add_source(config::File::with_name(settings_file_path.as_str()))
            .build()
            .unwrap();

        // initialize monitoring modules
        let mut monitors: Vec<Monitors> = vec![];
        if settings
            .get::<String>("usb_mon_enabled")
            .unwrap()
            .eq_ignore_ascii_case("true")
        {
            monitors.push(Monitors::USBMon(USBMon::new(settings.clone())));
        }
        // if settings
        //     .get::<String>("net_mon_enabled")
        //     .unwrap()
        //     .eq_ignore_ascii_case("true")
        // {
        //     monitors.push(Monitors::NetMon(NETMon::new(settings.clone())));
        // }
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
            monitors.push(Monitors::FileChanges(FileChanges::new(
                settings.clone(),
                blacklist_file_path,
                cache_dir,
            )));
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
                    // Monitors::NetMon(e) => {
                    //     // println!("{:#?}", e);
                    //     e.check().await;
                    // }
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