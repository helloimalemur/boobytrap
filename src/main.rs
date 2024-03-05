use crate::tw::AppState;

mod tw;
mod monitors;
mod actions;
mod notify;


#[tokio::main]
async fn main() {
    let mut app = AppState::new();
    app.run().await;
}
