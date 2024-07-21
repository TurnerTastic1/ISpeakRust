use crate::application::error::errors::ApplicationError;
use super::{Server, logger };

pub struct Application {
    server: Server,
}

impl Application {
    fn new(server: Server) -> Self {
        Application { server }
    }

    pub async fn initialize() -> Result<Self, ApplicationError> {
        logger::init_logging();

        let server = Server::initialize().await?;

        Ok(Application::new(server))
    }

    pub async fn run(&self) -> Result<(), ApplicationError> {
        self.server.start_server().await
    }
}