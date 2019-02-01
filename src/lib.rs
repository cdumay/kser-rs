#[deny(warnings)]
extern crate cdumay_error;
extern crate cdumay_job;
extern crate cdumay_result;
extern crate futures;
extern crate rdkafka;
extern crate rdkafka_sys;
extern crate serde_json;

#[macro_use]
extern crate log;

pub mod errors;
pub mod consumer;
pub mod producer;