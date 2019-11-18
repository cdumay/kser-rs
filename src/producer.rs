use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use cdumay_error::ErrorInfo;
use cdumay_job::{Message, MessageRepr};
use futures::future::Future;
use rdkafka::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::KserErrors;

pub trait Produce: Sized {
    type ErrorItem: ErrorInfo;

    fn new(config: &mut ClientConfig) -> Result<Producer, Self::ErrorItem>;
    fn from_file(filename: &str) -> Result<Producer, Self::ErrorItem>;
    fn bulk_send(&self, topic_name: &str, messages: Vec<MessageRepr>);
    fn send(&self, topic_name: &str, message: MessageRepr) {
        self.bulk_send(topic_name, vec![message]);
    }
}


pub struct Producer(FutureProducer);

impl Produce for Producer {
    type ErrorItem = KserErrors;

    fn new(config: &mut ClientConfig) -> Result<Producer, KserErrors> {
        let producer: FutureProducer = config.create()?;
        Ok(Producer(producer))
    }
    fn from_file(filename: &str) -> Result<Producer, KserErrors> {
        match Path::new(filename).exists() {
            false => Err(KserErrors::ConfigurationError(format!("Configuration file '{}' doesn't exists", filename))),
            true => {
                let content: HashMap<String, String> = serde_json::from_reader(&File::open(filename)?)?;
                let mut conf = ClientConfig::new();
                for (k, v) in content.iter() {
                    conf.set(k, v);
                }
                conf.set_log_level(RDKafkaLogLevel::Debug);
                Producer::new(&mut conf)
            }
        }
    }
    fn bulk_send(&self, topic_name: &str, messages: Vec<MessageRepr>) {
        let futures = messages.iter().map(|msg| {
            self.0.send(
                FutureRecord::to(topic_name)
                    .payload(&serde_json::to_string(&msg).unwrap())
                    .key(&format!("Key {}", msg.uuid())),
                0,
            ).map(move |delivery_status| {   // This will be executed onw the result is received
                info!("Delivery status for message {} received", msg.uuid());
                delivery_status
            })
        }).collect::<Vec<_>>();
        for future in futures {
            info!("Future completed. Result: {:?}", future.wait());
        }
    }
}
