mod server;
mod logger;

use server::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::initialize().await?;
    app.start_server().await
}
