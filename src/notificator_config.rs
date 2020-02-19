use agni_client::client::AgniClientConfig;
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NotificatorStrategiesConfig{
    AgniNotificator {
        agni_config: AgniClientConfig,
        environments: HashMap<String, String>,
        topic: String
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificatorConfig{
    pub strategy: NotificatorStrategiesConfig
}