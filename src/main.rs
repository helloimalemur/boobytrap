use crate::tw::AppState;

mod monitors;
mod tw;

#[tokio::main]
async fn main() {
    let mut app = AppState::new();
    app.run().await;
}
