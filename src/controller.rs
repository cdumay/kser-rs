use cdumay_error::ErrorBuilder;
use crate::task::Task;
use crate::errors::KserErrors;
use crate::messages::{Message, MessageProperties};
use crate::Result;
use crate::samples::{Hello, DiceRoll};

pub trait Register {
    fn execute(msg: &mut Message) -> Result;
}

pub struct BaseRegistry;

impl Register for BaseRegistry {
    fn execute(msg: &mut Message) -> Result {
        match msg.entrypoint().as_str() {
            "hello" => Hello::execute(msg),
            "roll" => DiceRoll::execute(msg),
            _ => Err(
                ErrorBuilder::from(KserErrors::NotFound)
                    .message(format!("Entrypoint '{}' is not registered", msg.entrypoint()))
                    .build()
            )
        }
    }
}

