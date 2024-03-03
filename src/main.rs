use crate::devices::USBMon;
use crate::tw::AppState;

mod devices;
mod tw;


fn main() {
    let app = AppState::new();
    // app.monitors.push(USBMon::new());
    app.run();
}
