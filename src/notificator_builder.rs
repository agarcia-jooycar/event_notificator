use crate::event_notificator::EventNotificator;
use crate::notificator_config::{NotificatorConfig, NotificatorStrategiesConfig};

use crate::strategies::agni_notificator::Notificator;

use std::boxed::Box;

pub struct NotificatorBuilder {}

impl NotificatorBuilder {
    pub fn from_strategies_config(config: NotificatorStrategiesConfig) -> EventNotificator {
        match config {
            NotificatorStrategiesConfig::AgniNotificator {
                agni_config,
                environments,
                topic,
            } => EventNotificator {
                notificator_strategy: Box::new(Notificator::new(agni_config, environments, topic)),
            },
        }
    }

    pub fn from_config(config: NotificatorConfig) -> EventNotificator {
        NotificatorBuilder::from_strategies_config(config.strategy)
    }
}
