use std::thread;

fn spawn_client_handler(message: String) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        println!("Thread started: {}", message);
        thread::sleep(std::time::Duration::from_secs(1));
        println!("Thread finished: {}", message);
    });

    handle
}

fn main() {
    // Array of messages
    let messages = ["Message 1", "Message 2", "Message 3"];
    let mut threads = Vec::new();

    // Loop to call spawn_client_handler for each message
    for &message in &messages {
        let handle = spawn_client_handler(message.to_string());
        threads.push(handle);
    }

    // Wait for all threads to finish
    for handle in threads {
        handle.join().unwrap();
    }
}
