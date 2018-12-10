use crate::Result;
use crate::task::Task;
use crate::messages::{Message, MessageProperties};
use serde_value::Value;
use rand::Rng;

pub struct Hello;

impl Task<Message> for Hello {
    fn required_fields() -> Option<Vec<String>> {
        Some(vec!["user".to_string()])
    }
    fn run(msg: &mut Message) -> Result {
        let default = "John Smith".to_string();
        let host = hostname::get_hostname().unwrap_or("localhost".to_string());

        let user = match msg.params().get("user") {
            Some(Value::String(data)) => data.clone(),
            _ => default,
        };
        *msg.result_mut().stdout_mut() = Some(format!("Hello {} from {}", user, host));
        Ok(())
    }
}

pub struct DiceRoll;

impl Task<Message> for DiceRoll {
    fn run(msg: &mut Message) -> Result {
        let turn = match msg.params().get("turn") {
            Some(Value::I32(value)) => value + 1,
            _ => match msg.result().search_value("turn", None) {
                Some(Value::I32(data)) => data + 1,
                _ => 1
            }
        };
        let num = rand::thread_rng().gen_range(1, 6);
        *msg.result_mut().stdout_mut() = Some(format!("Turn {}, you made a {}", turn, num));
        msg.result_mut().retval_mut().insert("turn".to_string(), Value::I32(turn));
        msg.result_mut().retval_mut().insert(format!("turn-{}", turn), Value::I32(num));
        Ok(())
    }
}