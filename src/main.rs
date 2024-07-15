use std::thread;

fn spawn_client_handler(message: String) {
    let handle = thread::spawn(move || {
        println!("Thread started: {}", message);
        thread::sleep(std::time::Duration::from_secs(1));
        println!("Thread finished: {}", message);
    });

    handle.join().unwrap();
}

fn main() {
    println!("Hello, world!");

    // Array of messages
    let messages = ["Message 1", "Message 2", "Message 3"];

    // Loop to call spawn_client_handler for each message
    for &message in &messages {
        spawn_client_handler(message.to_string());
    }
}
