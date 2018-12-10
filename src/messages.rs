use cdumay_result::{ExecResult, ExecResultBuilder};
use serde_value::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    uuid: uuid::Uuid,
    entrypoint: String,
    metadata: HashMap<String, Value>,
    params: HashMap<String, Value>,
    result: ExecResult,
}

pub trait MessageProperties {
    fn uuid(&self) -> &uuid::Uuid;
    fn uuid_mut(&mut self) -> &mut uuid::Uuid;

    fn entrypoint(&self) -> &String;
    fn entrypoint_mut(&mut self) -> &mut String;

    fn metadata(&self) -> &HashMap<String, Value>;
    fn metadata_mut(&mut self) -> &mut HashMap<String, Value>;

    fn params(&self) -> &HashMap<String, Value>;
    fn params_mut(&mut self) -> &mut HashMap<String, Value>;

    fn result(&self) -> &ExecResult;
    fn result_mut(&mut self) -> &mut ExecResult;
    fn as_result(&self) -> ExecResult;
}

impl Message {
    pub fn new(uuid: Option<uuid::Uuid>, entrypoint: &str, params: Option<HashMap<String, Value>>, result: Option<ExecResult>, metadata: Option<HashMap<String, Value>>) -> Message {
        let muuid = uuid.unwrap_or(uuid::Uuid::new_v4());
        let mut result = result.unwrap_or(ExecResultBuilder::new().uuid(muuid).build());
        *result.uuid_mut() = muuid;
        Message {
            entrypoint: entrypoint.to_string(),
            params: params.unwrap_or(HashMap::new()),
            metadata: metadata.unwrap_or(HashMap::new()),
            result,
            uuid: muuid,
        }
    }
}

impl MessageProperties for Message {
    fn uuid(&self) -> &uuid::Uuid { &self.uuid }
    fn uuid_mut(&mut self) -> &mut uuid::Uuid { &mut self.uuid }
    fn entrypoint(&self) -> &String { &self.entrypoint }
    fn entrypoint_mut(&mut self) -> &mut String { &mut self.entrypoint }
    fn metadata(&self) -> &HashMap<String, Value> { &self.metadata }
    fn metadata_mut(&mut self) -> &mut HashMap<String, Value> { &mut self.metadata }
    fn params(&self) -> &HashMap<String, Value> { &self.params }
    fn params_mut(&mut self) -> &mut HashMap<String, Value> { &mut self.params }
    fn result(&self) -> &ExecResult { &self.result }
    fn result_mut(&mut self) -> &mut ExecResult { &mut self.result }

    fn as_result(&self) -> ExecResult {
        let mut res = ExecResult::default();
        *res.uuid_mut() = *self.uuid();
        *res.retval_mut() = self.result().retval().clone();
        res
    }
}


impl Default for Message {
    fn default() -> Message {
        let uuid = uuid::Uuid::new_v4();
        Message {
            uuid,
            entrypoint: String::new(),
            metadata: HashMap::new(),
            params: HashMap::new(),
            result: ExecResultBuilder::new().uuid(uuid).build(),
        }
    }
}
