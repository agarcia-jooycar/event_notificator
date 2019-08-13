use serde_json::Value;
use serde_json::map::Map;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TripNotification {
    pub tags: Map<String, Value>,
    pub id: String,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    pub created: u64
}

impl TripNotification{
    fn get_tag_str(tag: &Value) -> String{
        let mut result = "".to_string();

        match tag {
            Value::String(value) => {
                result = value.clone();
            },
            Value::Number(value) => {
                result = value.to_string();
            },
            _ => {}
        };

        result
    }

    pub fn clean_tags(&self) -> Vec<(String, String)>{
        let mut result = Vec::new();

        for (key, tag) in &self.tags {
            match tag {
                Value::Array(values) => {
                    for value in values {
                        let tag_str = TripNotification::get_tag_str(value);
                        if tag_str != "".to_string(){
                            result.push((key.clone(), tag_str));
                        }
                    }
                },
                _ => {
                    let tag_str = TripNotification::get_tag_str(tag);
                    if tag_str != "".to_string(){
                        result.push((key.clone(), tag_str));
                    }
                }
            };
        }


        result
    }
}