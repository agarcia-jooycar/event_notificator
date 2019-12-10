use crate::notifications::data_package_notification::HeaderNotification;
use data_package_v2::data_package_v2::DataPackageV2;
use failure::Error;

pub trait NotificatorStrategy: Send + Sync {
    fn notify(&self, header: &HeaderNotification, data_package: &DataPackageV2) -> Result<(), Error>;
}