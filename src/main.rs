use crate::devices::USBMon;
use crate::tw::{AppState, Monitors};

mod devices;
mod tw;


fn main() {
    let mut app = AppState::new();
    app.monitors.push(Monitors::USB(USBMon::new()));
    app.run();
}
