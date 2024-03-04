use crate::devices::USBMon;
use crate::network::NETMon;
use crate::tw::{AppState, Monitors};

mod devices;
mod network;
mod tw;


#[tokio::main]
async fn main() {
    let mut app = AppState::new();
    // let mut binding = app.monitors.lock();
    // let app_lock = binding.as_mut().unwrap();
    //
    // app_lock.push(Monitors::USBMon(USBMon::new()));
    // app_lock.push(Monitors::NetMon(NETMon::new()));

    app.run().await;
}
