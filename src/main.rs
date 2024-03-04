use crate::devices::USBMon;
use crate::network::NETMon;
use crate::tw::{AppState, Monitors};

mod devices;
mod network;
mod tw;

#[tokio::main]
async fn main() {
    let mut app = AppState::new();
    app.run().await;
}
