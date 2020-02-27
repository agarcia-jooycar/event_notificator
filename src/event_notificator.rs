use crate::notificator_strategy::NotificatorStrategy;
use std::boxed::Box;

use crate::notifications::data_package_notification::HeaderNotification;
use data_package_v2::data_package_v2::DataPackageV2;

use failure::Error;

pub struct EventNotificator{
    pub notificator_strategy: Box<dyn NotificatorStrategy>
}

impl EventNotificator{
    pub async fn notify(&self, header: &HeaderNotification, data_package: &DataPackageV2) -> Result<(), Error>{
        self.notificator_strategy.notify(header, data_package).await
    }
}