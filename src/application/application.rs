use super::{ Server, logger };

pub struct Application {
    server: Server,
}

impl Application {
    fn new(server: Server) -> Self {
        Application { server }
    }

    pub async fn initialize() -> Result<Self, Box<dyn std::error::Error>> {
        logger::init_logging();

        let server = Server::initialize().await?;

        Ok(Application::new(server))
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.server.start_server().await
    }
}