use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use cdumay_error::ErrorInfo;
use cdumay_job::{Message, MessageRepr};
use futures::stream::Stream;
use rdkafka::ClientConfig;
use rdkafka::ClientContext;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::CommitMode;
use rdkafka::consumer::Consumer as rdConsumer;
use rdkafka::consumer::ConsumerContext;
use rdkafka::consumer::Rebalance;
use rdkafka::consumer::StreamConsumer;
use rdkafka::error::KafkaResult;
use rdkafka::Message as rdMessage;

use crate::KserErrors;

pub trait Registry {
    fn execute(&self, msg: &MessageRepr) -> cdumay_result::ResultRepr;
}

pub trait Consume: Sized {
    type ErrorItem: ErrorInfo;
    type RegistryItem: Registry;

    fn new(config: &mut ClientConfig, topics: Vec<&str>, registry: Self::RegistryItem) -> Result<Self, Self::ErrorItem>;
    fn from_file(filename: &str, topics: Vec<&str>, registry: Self::RegistryItem) -> Result<Self, Self::ErrorItem>;

    fn run(&self);
}


struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: *mut rdkafka_sys::RDKafkaTopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;

pub struct Consumer<R: Registry> {
    client: LoggingConsumer,
    registry: R,
}

impl<R: Registry> Consume for Consumer<R> {
    type ErrorItem = KserErrors;
    type RegistryItem = R;

    fn new(config: &mut ClientConfig, topics: Vec<&str>, registry: R) -> Result<Consumer<R>, KserErrors> {
        let client: LoggingConsumer = config.create_with_context(CustomContext)?;
        client.subscribe(&topics)?;
        Ok(Consumer { client, registry })
    }
    fn from_file(filename: &str, topics: Vec<&str>, registry: R) -> Result<Consumer<R>, KserErrors> {
        match Path::new(filename).exists() {
            false => Err(KserErrors::ConfigurationError(format!("Configuration file '{}' doesn't exists", filename))),
            true => {
                let reader = File::open(filename)?;
                let content: HashMap<String, String> = serde_json::from_reader(&reader)?;
                let mut conf = ClientConfig::new();
                for (k, v) in content.iter() {
                    conf.set(k, v);
                }
                conf.set_log_level(RDKafkaLogLevel::Debug);
                Consumer::new(&mut conf, topics, registry)
            }
        }
    }
    fn run(&self) {
        info!("Consumer starting...");
        let message_stream = self.client.start();
        for message in message_stream.wait() {
            match message {
                Err(_) => error!("Error while reading from stream."),
                Ok(Err(e)) => error!("Kafka error: {}", e),
                Ok(Ok(m)) => {
                    match m.payload_view::<str>() {
                        Some(Ok(s)) => match serde_json::from_str::<MessageRepr>(s) {
                            Ok(ref msg) => {
                                let res = self.registry.execute(msg);
                                debug!("Message {}: {}", msg.uuid(), res);
                            }
                            Err(err) => error!("Message deserializing error: {}", err),
                        },
                        None => error!("Empty message!"),
                        Some(Err(e)) => error!("Error while deserializing message payload: {}", e),
                    };
                    self.client.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        }
    }
}