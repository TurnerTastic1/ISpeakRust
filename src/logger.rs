use log4rs;

pub fn init_logging() {
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("Failed to initialize logging: {:?}", e);
    }
}
