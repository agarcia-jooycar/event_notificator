use toml::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetricNotification {
    pub start: i64,
    pub end: i64,
    pub start_value: Option<Value>,
    pub end_value: Option<Value>,
    pub metric_name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataMetricNotifications {
    pub key_ref: String,
    pub key_value: String,
    pub notifications: Vec<MetricNotification>

}