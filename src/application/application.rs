use super::Server;

pub struct Application {
    server: Server
}

impl Application {
    fn new(server: Server) -> Self {
        Application { server }
    }

    pub async fn initialize() -> Result<Self, Box<dyn std::error::Error>> {
        log4rs::init_file("log4rs.yaml", Default::default())?;

        let server = Server::initialize().await?;

        Ok(Application::new(server))
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.server.start_server().await
    }
}