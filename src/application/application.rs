use crate::application::model::error::ApplicationError;
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

        let server = match Server::initialize().await {
            Ok(server) => server,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Application::new(server))
    }

    pub async fn run(&self) -> Result<(), ApplicationError> {
        match self.server.start_server().await {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(e);
            }
        }
    }
}