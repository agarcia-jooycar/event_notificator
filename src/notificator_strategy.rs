use trip_concrete::trip::Trip;

use crate::notifications::metric_notification::DataMetricNotifications;
use crate::notifications::trip_notification::TripNotification;

use failure::Error;

pub trait NotificatorStrategy: Send + Sync {
    fn notify_metrics(&self, notifications: &DataMetricNotifications) -> Result<(), Error>;

    fn notify_trip(&self, trip: &Trip) -> Result<(), Error>;

    fn notify_trip_metadata(&self, trip_notification: &TripNotification) -> Result<(), Error>;
}