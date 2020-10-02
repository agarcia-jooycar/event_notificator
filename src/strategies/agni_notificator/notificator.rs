use crate::notifications::data_package_notification::{
    BodyNotification, DataPackageNotification, HeaderNotification,
};
use data_package_v2::data_package_v2::DataPackageV2;

use std::collections::HashMap;

use crate::errors::NotificationError;

use crate::notificator_strategy::NotificatorStrategy;
use agni_client::client::{AgniClient, AgniClientConfig};
use agni_client::topic::AgniTopic;
use std::sync::Arc;

use async_trait::async_trait;

#[derive(Debug)]
pub struct Notificator {
    topic_client: AgniTopic,
    environments_topics: HashMap<String, AgniTopic>,
}

impl Notificator {
    pub fn new(
        agni_config: AgniClientConfig,
        environments: HashMap<String, String>,
        topic: String,
    ) -> Notificator {
        let agni_client = Arc::new(AgniClient::new(&agni_config));
        let topic_client = AgniTopic::new(agni_client.clone(), topic);
        let environments_topics = environments
            .iter()
            .map(|(k, t)| (k.clone(), AgniTopic::new(agni_client.clone(), t.clone())))
            .collect::<HashMap<String, AgniTopic>>();

        Notificator {
            topic_client,
            environments_topics,
        }
    }

    async fn notify_msg(&self, data_package_notification: &DataPackageNotification, topic: &AgniTopic) -> Result<(), NotificationError>{
        let notifications_str =
            serde_json::to_string(&data_package_notification).map_err(|e| {
                NotificationError::MessageError {
                    details: e.to_string(),
                }
            })?;

        topic
            .publish(notifications_str.clone())
            .await
            .map_err(|e| NotificationError::ApiError {
                details: e.to_string(),
            })?;

        Ok(())
    }
}

#[async_trait]
impl NotificatorStrategy for Notificator {
    async fn notify(
        &self,
        header: &HeaderNotification,
        data_package: &DataPackageV2,
    ) -> Result<(), NotificationError> {
        let mut data_package_notification = DataPackageNotification {
            header: header.clone(),
            body: BodyNotification {
                data_package: data_package.clone(),
            },
        };

        if !data_package.tags.is_empty() {
            for tag in &data_package.tags {
                data_package_notification.body.data_package.tags = vec![tag.clone()];

                if let Some(topic_client) = self.environments_topics.get(&tag.environment) {
                    self.notify_msg(&data_package_notification, topic_client).await?;
                }
            }
        } else {
            self.notify_msg(&data_package_notification, &self.topic_client).await?;
        }

        Ok(())
    }
}
