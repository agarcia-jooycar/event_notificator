use toml::Value;
use metrics_tsdb::metric::metric_data::MetricData;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetricNotification {
    pub start: i64,
    pub end: i64,
    pub start_value: Option<Value>,
    pub end_value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MetricData>>,
    pub metric_name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataMetricNotifications {
    pub key_ref: String,
    pub key_value: String,
    pub notifications: Vec<MetricNotification>

}