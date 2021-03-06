use crate::notifications::data_package_notification::HeaderNotification;
use async_trait::async_trait;
use data_package_v2::data_package_v2::DataPackageV2;
use failure::Error;

#[async_trait]
pub trait NotificatorStrategy: Send + Sync {
    async fn notify(
        &self,
        header: &HeaderNotification,
        data_package: &DataPackageV2,
    ) -> Result<(), Error>;
}
