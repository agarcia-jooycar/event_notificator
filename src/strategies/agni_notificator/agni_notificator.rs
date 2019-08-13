use trip_concrete::trip::Trip;

use crate::notifications::metric_notification::DataMetricNotifications;
use crate::notifications::trip_notification::TripNotification;

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
    fn notify_metrics(&self, notifications: &DataMetricNotifications) -> Result<(), Error>{
        let notifications_str = serde_json::to_string(notifications).map_err(|e| {
            failure::err_msg(format!("Error parsing Notification to String. Details: {}", e.to_string()))
        })?;

        let mut topic = self.agni_client.topic(self.topic.clone());

        topic.publish(notifications_str.clone()).map_err(|e| {
            failure::err_msg(format!("Error publishing Notification {:?}. Details: {}", notifications_str, e.to_string()))
        })?;

        Ok(())
    }

    fn notify_trip(&self, trip: &Trip) -> Result<(), Error>{
        for tag in &trip.tags {
            let mut new_trip = trip.clone();
            new_trip.tags = vec![tag.clone()];
            let notifications_str = serde_json::to_string(&new_trip).map_err(|e| {
                failure::err_msg(format!("Error parsing Notification to String. Details: {}", e.to_string()))
            })?;

            if let Some(topic_name) = self.environments.get(&tag.name){
                let mut topic = self.agni_client.topic(topic_name.clone());

                topic.publish(notifications_str.clone()).map_err(|e| {
                    failure::err_msg(format!("Error publishing Notification {:?}. Details: {}", notifications_str, e.to_string()))
                })?;
            }
        }

        Ok(())
    }

    fn notify_trip_metadata(&self, trip_notification: &TripNotification) -> Result<(), Error>{
        let notifications_str = serde_json::to_string(trip_notification).map_err(|e| {
            failure::err_msg(format!("Error parsing Notification to String. Details: {}", e.to_string()))
        })?;

        let mut topic = self.agni_client.topic(self.topic.clone());

        topic.publish(notifications_str.clone()).map_err(|e| {
            failure::err_msg(format!("Error publishing Notification {:?}. Details: {}", notifications_str, e.to_string()))
        })?;

        Ok(())
    }
}