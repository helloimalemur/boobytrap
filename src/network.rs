use crate::tw::EventMonitor;
use crate::tw::*;

pub struct NETMon {
    triggered: bool,
    interfaces: Vec<String>,
}

impl NETMon {
    pub fn new() -> Self {
        NETMon {
            triggered: false,
            interfaces: vec![],
        }
    }
}

impl EventMonitor for NETMon {
    async fn check(&mut self) {
        println!("check net: {}", self.triggered);
        if let Ok(check) = httping::ping("koonts.net", "", "https", 443).await {
            self.triggered = check
        }
    }
}
