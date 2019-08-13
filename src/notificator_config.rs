use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io;
use toml;
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

impl NotificatorConfig {
    pub fn load(config_file_path: &Path) -> io::Result<NotificatorConfig> {
        let mut toml_file = File::open(&config_file_path)?;
        let mut config_str = String::new();
        toml_file.read_to_string(&mut config_str)?;

        let config: NotificatorConfig = toml::from_str(&config_str)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(config)
    }
}