use std::collections::BTreeMap;
use std::error::Error;

use cdumay_error::{ErrorInfo, ErrorType, GenericErrors, Registry};
use serde_value::Value;

pub struct KserTypeErrors;

impl KserTypeErrors {
    pub const KAFKA_ERROR: ErrorType = ErrorType(500, "Err-02932", "Kafka Error");
}

impl Registry for KserTypeErrors {
    fn from_msgid(msgid: &str) -> ErrorType {
        match msgid {
            "Err-02932" => KserTypeErrors::KAFKA_ERROR,
            _ => Self::default()
        }
    }
}

#[derive(Debug)]
pub enum KserErrors {
    IoError(std::io::Error),
    ConfigurationError(String),
    KafkaError(rdkafka::error::KafkaError),
    JSONError(serde_json::Error),
}

impl ErrorInfo for KserErrors {
    fn code(&self) -> u16 {
        match self {
            KserErrors::IoError(_) => GenericErrors::IO_ERROR.code(),
            KserErrors::ConfigurationError(_) => GenericErrors::INVALID_CONFIGURATION.code(),
            KserErrors::KafkaError(_) => KserTypeErrors::KAFKA_ERROR.code(),
            KserErrors::JSONError(_) => GenericErrors::SERIALIZATION_ERROR.code(),
        }
    }

    fn extra(&self) -> Option<BTreeMap<String, Value>> {
        None
    }

    fn message(&self) -> String {
        match self {
            KserErrors::IoError(err) => err.description().to_string(),
            KserErrors::ConfigurationError(err) => err.clone(),
            KserErrors::KafkaError(err) => err.description().to_string(),
            KserErrors::JSONError(err) => err.description().to_string(),
        }
    }

    fn msgid(&self) -> String {
        match self {
            KserErrors::IoError(_) => GenericErrors::IO_ERROR.msgid(),
            KserErrors::ConfigurationError(_) => GenericErrors::INVALID_CONFIGURATION.msgid(),
            KserErrors::KafkaError(_) => KserTypeErrors::KAFKA_ERROR.msgid(),
            KserErrors::JSONError(_) => GenericErrors::SERIALIZATION_ERROR.msgid(),
        }
    }
}

impl From<std::io::Error> for KserErrors {
    fn from(err: std::io::Error) -> KserErrors {
        KserErrors::IoError(err)
    }
}

impl From<rdkafka::error::KafkaError> for KserErrors {
    fn from(err: rdkafka::error::KafkaError) -> KserErrors {
        KserErrors::KafkaError(err)
    }
}

impl From<serde_json::Error> for KserErrors {
    fn from(err: serde_json::Error) -> KserErrors {
        KserErrors::JSONError(err)
    }
}