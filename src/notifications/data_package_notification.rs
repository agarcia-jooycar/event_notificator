use data_package_v2::data_package_v2::DataPackageV2;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HeaderNotification {
    pub created: u64,
    pub microservice: String,
    pub metadata: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BodyNotification {
    #[serde(rename = "dataPackage")]
    pub data_package: DataPackageV2,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataPackageNotification {
    pub header: HeaderNotification,
    pub body: BodyNotification,
}
