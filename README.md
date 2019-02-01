# kser

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Library to serialize, deserialize and execute tasks using Kafka.

## Example

The following example will send a `Hello` task to kafka and will be executed by 
a consumer. See the example given on the dependency [cdumay_job](https://github.com/cdumay/cdumay-job-rs).

```toml
[dependencies]
cdumay_error = { git = "https://github.com/cdumay/cdumay-errors-rs" , features = ["http"] }
cdumay_result = { git = "https://github.com/cdumay/cdumay-result-rs", features = ["cdumay-error"]}
cdumay_job = { git = "https://github.com/cdumay/cdumay-job-rs" }
env_logger = "0.6"
hostname = "0.1"
kser = { git = "https://github.com/cdumay/kser-rs" }
log = "0.4"
rand = "0.6"
serde-value = "0.5"

[[bin]]
name = "kser-producer"
path = "bin/producer.rs"

[[bin]]
name = "kser-consumer"
path = "bin/consumer.rs"
```

**src/lib.rs**

```rust
extern crate cdumay_error;
extern crate cdumay_job;
extern crate cdumay_result;
extern crate env_logger;
extern crate hostname;
extern crate rand;
extern crate serde_value;
```

**bin/producer.rs**

```rust
#[macro_use]
extern crate log;

use cdumay_job::messages::MessageRepr;
use kser::producer::Producer;
use serde_value::Value;
use std::collections::HashMap;

fn main() {
    env_logger::init();

    match Producer::from_file("/tmp/producer.json") {
        Ok(producer) => producer.send(
            "my.topic",
            MessageRepr::new(
                None,
                "hello",
                {
                    let mut params: HashMap<String, Value> = HashMap::new();
                    params.insert("user".to_string(), Value::String("Cedric".to_string()));
                    Some(params)
                },
                None,
                None,
            ),
        ),
        Err(err) => error!("{}", err),
    };
}
```

**bin/consumer.rs**

```rust
#[macro_use]
extern crate log;

use cdumay_error::{ErrorRepr,ErrorReprBuilder};
use cdumay_error::http::HttpErrors;
use cdumay_job::messages::MessageRepr;
use cdumay_job::status::Status;
use cdumay_job::task::Task;
use cdumay_result::{ResultRepr, ResultProps};
use kser::consumer::{Registry, Consumer};
use serde_value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Hello {
    message: MessageRepr,
    status: Status,
    result: ResultRepr,
}

impl Task for Hello {
    fn new(message: &MessageRepr, result: Option<ResultRepr>) -> Hello {
        Hello {
            message: message.clone(),
            result: result.unwrap_or(message.result().clone()),
            status: Status::Pending,
        }
    }
    fn status(&self) -> Status { self.status.clone() }
    fn status_mut(&mut self) -> &mut Status { &mut self.status }

    fn message(&self) -> MessageRepr { self.message.clone() }
    fn message_mut(&mut self) -> &mut MessageRepr { &mut self.message }
    fn result(&self) -> ResultRepr { self.result.clone() }
    fn result_mut(&mut self) -> &mut ResultRepr { &mut self.result }

    fn required_params<'a>() -> Option<Vec<&'a str>> {
        Some(vec!["user"])
    }
    fn run(&mut self) -> Result<ResultRepr, ErrorRepr> {
        let default = "John Smith".to_string();
        let host = hostname::get_hostname().unwrap_or("localhost".to_string());

        let user = match self.search_data("user") {
            Some(Value::String(data)) => data.clone(),
            _ => default,
        };
        Ok({
            let mut res = ResultRepr::from(&self.message());
            *res.stdout_mut() = Some(format!("Hello {} from {}", user, host));
            res
        })
    }
}

struct BaseRegistry;

impl Registry for BaseRegistry {
    fn execute(&self, msg: &MessageRepr) -> ResultRepr {
        match msg.entrypoint().as_str() {
            "hello" => Hello::new(msg, None).execute(None),
            _ => ResultRepr::from(
                ErrorReprBuilder::new(HttpErrors::NOT_FOUND)
                    .message(format!("Entrypoint '{}' is not registered", msg.entrypoint()))
                    .build()
            )
        }
    }
}

fn main() {
    env_logger::init();

    match Consumer::from_file("/tmp/consumer.json", vec!["my.topic"], BaseRegistry {}) {
        Err(err) => error!("{}", err),
        Ok(consumer) => consumer.run()
    };
}
```

### Producer output (RUST_LOG=info)
```
[2019-02-01T16:50:52Z INFO  kser::producer] Delivery status for message 13ec8caf-9a3d-4a7d-99fc-67a586343fe3 received
[2019-02-01T16:50:52Z INFO  kser::producer] Future completed. Result: Ok(Ok((0, 168)))
```

### Consumer output (RUST_LOG=debug)
```
[2019-02-01T16:50:40Z INFO  kser::consumer] Consumer starting...
[2019-02-01T16:50:41Z INFO  kser::consumer] Pre rebalance Assign(TPL {(my.topic, 0): Invalid, (my.topic, 1): Invalid, (my.topic, 2): Invalid, })
[2019-02-01T16:50:41Z INFO  kser::consumer] Post rebalance Assign(TPL {(my.topic, 0): Invalid, (my.topic, 1): Invalid, (my.topic, 2): Invalid, })
[2019-02-01T16:50:52Z DEBUG cdumay_job::task] hello[13ec8caf-9a3d-4a7d-99fc-67a586343fe3] - PreRun
[2019-02-01T16:50:52Z DEBUG cdumay_job::task] hello[13ec8caf-9a3d-4a7d-99fc-67a586343fe3] - SetStatus: status updated 'PENDING' -> 'RUNNING'
[2019-02-01T16:50:52Z DEBUG cdumay_job::task] hello[13ec8caf-9a3d-4a7d-99fc-67a586343fe3] - Run: Result: Ok(0, stdout: None)
[2019-02-01T16:50:52Z DEBUG cdumay_job::task] hello[13ec8caf-9a3d-4a7d-99fc-67a586343fe3] - PostRun: Result: Ok(0, stdout: Some("Hello Cedric from cdumay-desk"))
[2019-02-01T16:50:52Z DEBUG cdumay_job::task] hello[13ec8caf-9a3d-4a7d-99fc-67a586343fe3] - SetStatus: status updated 'RUNNING' -> 'SUCCESS'
[2019-02-01T16:50:52Z INFO  cdumay_job::task] hello[13ec8caf-9a3d-4a7d-99fc-67a586343fe3] - Success: Result: Ok(0, stdout: Some("Hello Cedric from cdumay-desk"))
[2019-02-01T16:50:52Z DEBUG kser::consumer] Message 13ec8caf-9a3d-4a7d-99fc-67a586343fe3: Result: Ok(0, stdout: Some("Hello Cedric from cdumay-desk"))
[2019-02-01T16:50:52Z INFO  kser::consumer] Committing offsets: Ok(())
```

## Configuration

Configurations are stored in JSON files where keys and values MUST be strings. 
Keys can be found on [librdkafka configuration documentation](https://github.com/edenhill/librdkafka/blob/master/CONFIGURATION.md).

**Comsumer example**
```json
{
  "bootstrap.servers": "kafka.example.com:9093,kafka-2.example.com:9093",
  "security.protocol": "sasl_ssl",
  "sasl.mechanisms": "PLAIN",
  "sasl.username": "test",
  "sasl.password": "1234",
  "group.id": "hello.group",
  "enable.auto.commit": "true",
  "enable.partition.eof": "false",
  "session.timeout.ms": "6000"
}
```


## Dependencies Links

- **cdumay_result**: https://github.com/cdumay/cdumay-result-rs
- **cdumay_error**: https://github.com/cdumay/cdumay-errors-rs
- **cdumay_job**: https://github.com/cdumay/cdumay-job-rs
- **librdkafka**: https://github.com/edenhill/librdkafka

## Project Links

- Issues: https://github.com/cdumay/kser-rs/issues
- Documentation: not available yet
