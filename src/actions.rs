use std::io::BufRead;
use std::process::Command;

pub async fn reboot_system() {
    if let Ok(reboot) = Command::new("reboot").output() {
        println!("{:#?}", reboot)
    }
}

pub async fn unmount_encrypted_volumes() {
    if let Ok(output) = Command::new("dmsetup").arg("ls").output() {
        let encrypted_vols = String::from_utf8(output.stdout.to_vec()).unwrap();
        for ea in encrypted_vols.lines() {
            let vol_split: Vec<_> = ea.split_ascii_whitespace().collect();
            let vol = vol_split.get(0).unwrap().to_string();
            println!("{}", vol);
            // let command_str = format!("findmnt -l | grep {}", vol);
            // println!("{}", command_str);
            let res = Command::new("cat")
                .arg("/proc/mounts")
                .arg("|")
                .arg("grep")
                .arg(vol)
                .output();
            println!("{:#?}", res);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::actions::unmount_encrypted_volumes;

    #[test]
    fn test_unmount() {
        let rt = tokio::runtime::Runtime::new();
        rt.unwrap().block_on(unmount_encrypted_volumes());
    }
}
