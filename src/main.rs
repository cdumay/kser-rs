#![feature(try_trait)]
extern crate cdumay_error;
extern crate cdumay_result;
extern crate serde_value;
extern crate env_logger;
extern crate hostname;
extern crate rand;
extern crate rdkafka;
extern crate serde;
extern crate serde_json;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;


pub mod controller;
pub mod errors;
pub mod kafka;
pub mod messages;
pub mod samples;
pub mod task;


pub type Result = std::result::Result<(), cdumay_error::Error>;

fn main() {
    use crate::controller::Register;
    use crate::messages::Message;
    use crate::messages::MessageProperties;

    env_logger::init();

//    let mut params = HashMap::new();
//    params.insert("user".to_string(), Value::from("Joe"));
//    let mut msg = Message::new(None, "hello", Some(params), None, None);
//
//    controller::BaseRegistry::execute(&mut msg);
//    println!("Result {}", serde_json::to_string_pretty(&msg.result()).unwrap());

    let mut t1 = Message::new(None, "roll", None, None, None);
    controller::BaseRegistry::execute(&mut t1).unwrap();
    println!("Result {}", serde_json::to_string_pretty(&t1.result()).unwrap());

    let mut t2 = Message::new(None, "roll", None, Some(t1.result().clone()), None);
    controller::BaseRegistry::execute(&mut t2).unwrap();
    println!("Result {}", serde_json::to_string_pretty(&t2.result()).unwrap());
    let mut t3 = Message::new(None, "roll", None, Some(t2.result().clone()), None);
    controller::BaseRegistry::execute(&mut t3).unwrap();
    println!("Result {}", serde_json::to_string_pretty(&t3.result()).unwrap());

}