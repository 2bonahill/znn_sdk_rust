use serde_json::Map;
use serde_json::Value;

pub struct AccountInfo {}

impl AccountInfo {
    pub fn fromJson(json: Map<String, Value>) -> Self {
        Self {}
    }
}
