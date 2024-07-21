mod application;

use log::debug;
use application::Application;
use application::error::errors::ApplicationError;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let app = match Application::initialize().await {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to initialize application: {:?}", e);
            return Err(e);
        }
    };

    match app.run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            debug!("Failed to start application: {:?}", e);
            Err(e)
        }
    }
}
