use serde_json::Value;
use data_package_v2::data_package_v2::DataPackageV2;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HeaderNotification {
    pub create: u64,
    pub microservice: String,
    pub metadata: Value
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BodyNotification {
    #[serde(rename = "dataPackage")]
    pub data_package: DataPackageV2
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataPackageNotification {
    pub header: HeaderNotification,
    pub body: BodyNotification
}