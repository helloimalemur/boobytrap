use crate::boobytrap::AppState;

mod boobytrap;
mod default_config;
mod monitors;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut app = AppState::new();
    app.config(args);
    app.run().await;
}
