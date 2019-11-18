#[deny(warnings)]
extern crate cdumay_error;
extern crate cdumay_job;
extern crate cdumay_result;
extern crate futures;
#[macro_use]
extern crate log;
extern crate rdkafka;
extern crate rdkafka_sys;
extern crate serde_json;
extern crate serde_value;

pub use consumer::{Consume, Consumer, Registry};
pub use errors::{KserErrors, KserTypeErrors};
pub use producer::{Produce, Producer};

mod errors;
mod consumer;
mod producer;