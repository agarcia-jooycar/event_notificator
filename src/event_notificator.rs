use crate::notificator_strategy::NotificatorStrategy;
use std::boxed::Box;

use trip_concrete::trip::Trip;

use crate::notifications::metric_notification::DataMetricNotifications;
use crate::notifications::trip_notification::TripNotification;

use failure::Error;

pub struct EventNotificator{
    pub notificator_strategy: Box<NotificatorStrategy>
}

impl EventNotificator{
    pub fn notify_metrics(&self, notifications: &DataMetricNotifications) -> Result<(), Error>{
        self.notificator_strategy.notify_metrics(notifications)
    }

    pub fn notify_trip(&self, trip: &Trip) -> Result<(), Error>{
        self.notificator_strategy.notify_trip(trip)

    }

    pub fn notify_trip_metadata(&self, trip_notification: &TripNotification) -> Result<(), Error>{
        self.notificator_strategy.notify_trip_metadata(trip_notification)
    }
}