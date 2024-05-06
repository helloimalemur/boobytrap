use crate::boobytrap::AppState;

mod monitors;
mod boobytrap;
mod default_config;

#[tokio::main]
async fn main() {
    let mut app = AppState::new();
    app.config();
    app.run().await;
}
