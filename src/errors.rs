use cdumay_error::{ErrorType, Registry};

pub struct KserErrors;

impl KserErrors {
    pub const CONFIGURATION_ERROR: ErrorType = ErrorType(500, "Err-00008", "Configuration Error");
    pub const KAFKA_ERROR: ErrorType = ErrorType(500, "Err-00674", "Kafka Error");
}

impl Registry for KserErrors {
    fn from_msgid(msgid: &str) -> ErrorType {
        match msgid {
            "Err-00008" => Self::CONFIGURATION_ERROR,
            "Err-00674" => Self::KAFKA_ERROR,
            _ => Self::default()
        }
    }
}