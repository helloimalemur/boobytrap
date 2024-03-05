use crate::tw::AppState;

mod devices;
mod network;
mod tw;
mod actions;
mod notify;

#[tokio::main]
async fn main() {
    let mut app = AppState::new();
    app.run().await;
}
