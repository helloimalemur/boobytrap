use crate::tw::EventMonitor;
use crate::tw::*;

pub struct NETMon {
    triggered: bool,
    interfaces: Vec<String>
}

impl NETMon {
    pub fn new() -> Self {
        NETMon { triggered: false, interfaces: vec![] }
    }

}

impl EventMonitor for NETMon {
    async fn check(&self) {
        println!("check net");
        let check = httping::ping("koonts.net", "", "https", 443).await;
    }
}
