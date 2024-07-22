pub mod application;
pub mod model;
mod server;
mod logger;
mod tests;

pub use self::application::Application;

use server::Server;
