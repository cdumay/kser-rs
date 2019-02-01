use cdumay_error::{ErrorRepr, ErrorReprBuilder};
use cdumay_job::messages::MessageRepr;
use crate::errors::KserErrors;
use futures::future::Future;
use rdkafka::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub struct Producer(FutureProducer);

impl Producer {
    pub fn from_file(filename: &str) -> Result<Producer, ErrorRepr> {
        match Path::new(filename).exists() {
            false => Err(ErrorReprBuilder::new(KserErrors::CONFIGURATION_ERROR)
                .message(format!("Configuration file '{}' doesn't exists", filename))
                .build()
            ),
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
    pub fn new(config: &mut ClientConfig) -> Result<Producer, ErrorRepr> {
        let producer: FutureProducer = match config.create() {
            Err(err) => return Err(ErrorReprBuilder::new(KserErrors::CONFIGURATION_ERROR)
                .message(format!("Failed to initialize producer: {}", err))
                .build()
            ),
            Ok(data) => data
        };
        Ok(Producer(producer))
    }
    pub fn bulk_send(&self, topic_name: &str, messages: Vec<MessageRepr>) {
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
    pub fn send(&self, topic_name: &str, message: MessageRepr) {
        self.bulk_send(topic_name, vec![message]);
    }
}
