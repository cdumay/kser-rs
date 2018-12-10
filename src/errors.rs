use cdumay_error::{Error, ErrorBuilder};

pub enum KserErrors {
    ConfigurationError,
    InternalError,
    IOError,
    NotFound,
    NotImplemented,
    ValidationError,
}

impl KserErrors {
    pub fn message(&self) -> String {
        match self {
            KserErrors::ConfigurationError => "Configuration error".to_string(),
            KserErrors::InternalError => "Internal Error".to_string(),
            KserErrors::IOError => "I/O Error".to_string(),
            KserErrors::NotFound => "Not Found".to_string(),
            KserErrors::NotImplemented => "Not Implemented".to_string(),
            KserErrors::ValidationError => "Validation error".to_string(),
        }
    }
    pub fn msgid(&self) -> String {
        match self {
            KserErrors::ConfigurationError => "ERR-19036".to_string(),
            KserErrors::InternalError => "ERR-29885".to_string(),
            KserErrors::IOError => "ERR-27582".to_string(),
            KserErrors::NotFound => "ERR-08414".to_string(),
            KserErrors::NotImplemented => "ERR-04766".to_string(),
            KserErrors::ValidationError => "ERR-04413".to_string(),
        }
    }
    pub fn code(&self) -> u16 {
        match self {
            KserErrors::ConfigurationError => 500,
            KserErrors::InternalError => 500,
            KserErrors::IOError => 500,
            KserErrors::NotFound => 404,
            KserErrors::NotImplemented => 501,
            KserErrors::ValidationError => 400,
        }
    }
}

impl From<KserErrors> for Error {
    fn from(err: KserErrors) -> Error {
        ErrorBuilder::from(err).build()
    }
}

impl From<KserErrors> for ErrorBuilder {
    fn from(err: KserErrors) -> ErrorBuilder {
        ErrorBuilder::new(&err.msgid())
            .code(err.code())
            .message(err.message().clone())
    }
}