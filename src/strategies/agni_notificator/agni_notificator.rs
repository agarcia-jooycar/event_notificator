use crate::notifications::data_package_notification::{ DataPackageNotification, HeaderNotification, BodyNotification };
use data_package_v2::data_package_v2::DataPackageV2;

use std::collections::HashMap;

use failure::Error;

use crate::notificator_strategy::NotificatorStrategy;
use agni_client::client::{AgniClient, AgniClientConfig};

#[derive(Debug)]
pub struct AgniNotificator {
    agni_client: AgniClient,
    environments: HashMap<String, String>,
    topic: String
}

impl AgniNotificator {
    pub fn new(agni_config: AgniClientConfig, environments: HashMap<String, String>, topic: String) -> AgniNotificator {
        let agni_client = AgniClient::new(&agni_config);

        AgniNotificator {
            agni_client,
            environments,
            topic
        }
    }
}

impl NotificatorStrategy for AgniNotificator {
    fn notify(&self, header: &HeaderNotification, data_package: &DataPackageV2) -> Result<(), Error>{
        let mut data_package_notification = DataPackageNotification{
            header: header.clone(),
            body: BodyNotification{
                data_package: data_package.clone()
            }
        };

        if !data_package.tags.is_empty(){
            for tag in &data_package.tags {
                data_package_notification.body.data_package.tags = vec![tag.clone()];

                let notifications_str = serde_json::to_string(&data_package_notification).map_err(|e| {
                    failure::err_msg(format!("Error parsing Notification to String. Details: {}", e.to_string()))
                })?;

                if let Some(topic_name) = self.environments.get(&tag.environment){
                    let mut topic = self.agni_client.topic(topic_name.clone());

                    topic.publish(notifications_str.clone()).map_err(|e| {
                        failure::err_msg(format!("Error publishing Notification {:?}. Details: {}", notifications_str, e.to_string()))
                    })?;
                }
            }
        }else{
            let notifications_str = serde_json::to_string(&data_package_notification).map_err(|e| {
                failure::err_msg(format!("Error parsing Notification to String. Details: {}", e.to_string()))
            })?;

            let mut topic = self.agni_client.topic(self.topic.clone());

            topic.publish(notifications_str.clone()).map_err(|e| {
                failure::err_msg(format!("Error publishing Notification {:?}. Details: {}", notifications_str, e.to_string()))
            })?;
        }

        Ok(())
    }
}