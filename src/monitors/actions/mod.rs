use crate::monitors::notify::send_discord;
use std::collections::HashMap;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub async fn reboot_system(settings_map: HashMap<String, String>) {
    let _res = send_discord("System rebooting", settings_map).await;
    if let Ok(reboot) = Command::new("reboot").output() {
        println!("{:#?}", reboot)
    }
}

pub async fn unmount_encrypted_volumes() {
    if let Ok(output) = Command::new("dmsetup").arg("ls").output() {
        let encrypted_vols = String::from_utf8(output.stdout.to_vec()).unwrap();
        for ea in encrypted_vols.lines() {
            let vol_split: Vec<_> = ea.split_ascii_whitespace().collect();
            let luks_vol = vol_split.get(0).unwrap().to_string();
            // println!("{}", luks_vol);
            // println!("{}", command_str);
            if let Ok(res) = Command::new("cat").arg("/proc/mounts").output() {
                let mount_file_results = String::from_utf8(res.stdout.to_vec()).unwrap();
                for mount in mount_file_results.lines() {
                    let luks_vol_str = luks_vol.as_str();
                    if mount.contains(luks_vol_str) {
                        let mount_split: Vec<&str> = mount.split_ascii_whitespace().collect();
                        let vol_mount_path = mount_split.get(1).unwrap();
                        // println!("{}", vol_mount_path);
                        commit_umount_volume(vol_mount_path);
                        thread::sleep(Duration::new(1, 0));
                        commit_luks_close(luks_vol_str);
                    }
                }
            }
        }
    }
}

fn commit_umount_volume(vol_mount_path: &str) {
    if let Ok(res) = Command::new("umount").arg(vol_mount_path).output() {
        println!("{}", String::from_utf8(res.stdout.to_vec()).unwrap())
    }
}

fn commit_luks_close(luks_vol: &str) {
    println!("close luks");
    if let Ok(res) = Command::new("cryptsetup")
        .arg("luksClose")
        .arg(luks_vol)
        .output()
    {
        println!("{}", String::from_utf8(res.stdout.to_vec()).unwrap())
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::monitors::actions::unmount_encrypted_volumes;
//
//     #[test]
//     fn test_unmount_encrypted() {
//         let rt = tokio::runtime::Runtime::new();
//         rt.unwrap().block_on(unmount_encrypted_volumes());
//     }
// }
