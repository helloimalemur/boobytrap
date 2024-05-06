use std::process::exit;

pub fn write_default_config(path: String) {
    println!("{path}");
    exit(1)
}
