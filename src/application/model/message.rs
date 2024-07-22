
#[derive(Debug)]
pub struct Message {
    pub message: String,
}

impl Message {
    pub fn new(message: String) -> Self {
        Message {
            message
        }
    }
}
