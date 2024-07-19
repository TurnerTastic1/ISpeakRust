mod logger;
mod application;

use application::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::initialize().await?;
    app.start().await
}
