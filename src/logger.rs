use log4rs;
use log::warn;

pub fn init_logging() {
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        warn!("Failed to initialize logging: {:?}", e);
    }
}
