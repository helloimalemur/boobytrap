use crate::boobytrap::AppState;

mod boobytrap;
mod default_config;
mod monitors;

#[tokio::main]
async fn main() {
    let mut app = AppState::new();
    app.config();
    app.run().await;
}
