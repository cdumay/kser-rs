use rdkafka::ClientConfig;
use rdkafka::ClientContext;
use rdkafka::consumer::Consumer as rdConsumer;
use rdkafka::consumer::ConsumerContext;
use rdkafka::consumer::Rebalance;
use rdkafka::consumer::StreamConsumer;
use rdkafka::error::KafkaResult;
use futures::stream::Stream;
use rdkafka::consumer::CommitMode;
use rdkafka::Message as rdMessage;
use std::path::Path;
use crate::errors::KserErrors;
use std::fs::File;
use std::collections::HashMap;
use rdkafka::config::RDKafkaLogLevel;
use cdumay_error::ErrorRepr;
use cdumay_error::ErrorReprBuilder;
use cdumay_job::messages::MessageRepr;

pub trait Registry {
    fn execute(&self, msg: &MessageRepr) -> cdumay_result::ResultRepr;
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

impl<R: Registry> Consumer<R> {
    pub fn from_file(filename: &str, topics: Vec<&str>, registry: R) -> Result<Consumer<R>, ErrorRepr> {
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
                Consumer::new(&mut conf, topics, registry)
            }
        }
    }
    pub fn new(config: &mut ClientConfig, topics: Vec<&str>, registry: R) -> Result<Consumer<R>, ErrorRepr> {
        let client: LoggingConsumer = match config.create_with_context(CustomContext) {
            Err(err) => return Err(ErrorReprBuilder::new(KserErrors::CONFIGURATION_ERROR)
                .message(format!("Failed to initialize consumer: {}", err))
                .build()
            ),
            Ok(data) => data
        };
        match client.subscribe(&topics) {
            Err(_) => Err(ErrorReprBuilder::new(KserErrors::CONFIGURATION_ERROR)
                .message(format!("Failed to subscribe to topic(s): {:?}", &topics))
                .build()
            ),
            Ok(_) => Ok(Consumer { client, registry })
        }
    }
    pub fn run(&self) {
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