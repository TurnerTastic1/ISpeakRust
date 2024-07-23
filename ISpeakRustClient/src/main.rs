use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("My CLI")
        .arg(Arg::with_name("server")
            .long("server")
            .value_name("ADDRESS:PORT")
            .required(true)
            .help("Server address (e.g., 127.0.0.1:8080)"))
        // Add other arguments and subcommands
        .get_matches();

    let server_address = matches.value_of("server").unwrap(); // Parse this into a SocketAddr

    // Now execute the appropriate command based on user input
    // Example: connect_to_server(server_address)?;

    Ok(())
}
