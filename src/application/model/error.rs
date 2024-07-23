#[derive(Debug)]
pub enum ErrorSeverity {
    WARN,
    ERROR,
    CRITICAL,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ApplicationError {
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error + Send>>,
    pub severity: ErrorSeverity,
}

impl ApplicationError {
    pub fn new(message: &str, cause: Option<Box<dyn std::error::Error + Send>>, severity: ErrorSeverity) -> Self {
        ApplicationError {
            message: message.to_string(),
            cause,
            severity,
        }
    }
}