EVENT NOTIFICATOR

Define the strategy to write messages to the Messaging System.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
event_notificator = { git = "ssh://github.com/jooycar/event_notificator.git", tag = "0.1.0" }
```

Then you can use it as follows. Create a `main.rs` and type:

```rust
extern crate data_package_v2;
extern crate event_notificator;
extern crate agni_client;
extern crate tokio;

use tokio::prelude::*;

use data_package_v2::data_package_v2::DataPackageV2;
use agni_client::client::AgniClientConfig;
use event_notificator::notificator_config::NotificatorStrategiesConfig;
use event_notificator::notificator_builder::NotificatorBuilder;
use event_notificator::notifications::data_package_notification::HeaderNotification;

#[tokio::main]
pub fn main(){
    let config = NotificatorStrategiesConfig::AgniNotificator {
        topic: "event_notificator_test".to_string(),
        environments: HashMap::new(),
        agni_config: AgniClientConfig::new("https://".to_string(), "event_notificator".to_string())
    };

    let event_notificator = NotificatorBuilder::from_strategies_config(config);

    let data_package_v2_str = r#"{"id":"DataPackage-imei-356363051883173-1574789395149","type":"dataRaw","created":1574376642000,"dataSource":{"type":"imei","value":"356363051883173"},"dataPoints":[{"time":1574376567000,"data":{"tripStart":{"occur":true,"timezone":3.0,"odometer":123.4}}},{"time":1574376568000,"data":{"speed":{"value":23.0},"acceleration":{"x":-0.0625,"y":0.03125,"z":1.015625}}},{"time":1574376568166,"data":{"acceleration":{"x":-0.046875,"y":0.046875,"z":1.0}}},{"time":1574376568332,"data":{"acceleration":{"x":-0.046875,"y":0.03125,"z":1.0}}},{"time":1574376568498,"data":{"acceleration":{"x":-0.03125,"y":0.078125,"z":0.953125}}},{"time":1574376568664,"data":{"acceleration":{"x":-0.046875,"y":0.046875,"z":1.0625}}},{"time":1574376568830,"data":{"acceleration":{"x":0.03125,"y":0.046875,"z":0.984375}}},{"time":1574376568996,"data":{"acceleration":{"x":0.015625,"y":0.03125,"z":1.015625}}},{"time":1574376569000,"data":{"speed":{"value":20.0}}},{"time":1574376569162,"data":{"acceleration":{"x":-0.015625,"y":0.015625,"z":1.0}}},{"time":1574376569328,"data":{"acceleration":{"x":-0.046875,"y":0.046875,"z":1.0}}},{"time":1574376569494,"data":{"acceleration":{"x":-0.046875,"y":0.03125,"z":1.03125}}},{"time":1574376569660,"data":{"acceleration":{"x":-0.0625,"y":0.046875,"z":1.03125}}},{"time":1574376569826,"data":{"acceleration":{"x":-0.03125,"y":0.03125,"z":0.953125}}},{"time":1574376570000,"data":{"speed":{"value":21.0},"acceleration":{"x":-0.046875,"y":0.03125,"z":1.046875}}},{"time":1574376570250,"data":{"acceleration":{"x":-0.046875,"y":0.078125,"z":1.0}}},{"time":1574376570500,"data":{"acceleration":{"x":-0.03125,"y":0.0625,"z":1.03125}}},{"time":1574376570750,"data":{"acceleration":{"x":0.015625,"y":0.0625,"z":1.0}}},{"time":1574376571000,"data":{"acceleration":{"x":0.03125,"y":0.03125,"z":1.015625},"speed":{"value":18.0}}},{"time":1574376571250,"data":{"acceleration":{"x":0.0,"y":0.03125,"z":1.046875}}},{"time":1574376571500,"data":{"acceleration":{"x":-0.03125,"y":0.03125,"z":1.0}}},{"time":1574376571750,"data":{"acceleration":{"x":-0.015625,"y":0.0625,"z":0.96875}}},{"time":1574376572000,"data":{"location":{"lat":-12.061495,"lng":-77.096108,"hDop":1,"satNum":7},"speed":{"value":17.0}}},{"time":1574376573000,"data":{"speed":{"value":16.0}}},{"time":1574376574000,"data":{"speed":{"value":15.0}}},{"time":1574376575000,"data":{"speed":{"value":15.0}}},{"time":1574376576000,"data":{"speed":{"value":15.0}}},{"time":1574376577000,"data":{"speed":{"value":15.0},"location":{"lat":-12.061697,"lng":-77.096078,"hDop":0,"satNum":7}}},{"time":1574376578000,"data":{"speed":{"value":13.0}}},{"time":1574376579000,"data":{"speed":{"value":10.0}}},{"time":1574376580000,"data":{"speed":{"value":8.0}}},{"time":1574376581000,"data":{"speed":{"value":8.0}}},{"time":1574376582000,"data":{"location":{"lat":-12.06184,"lng":-77.096082,"hDop":0,"satNum":7},"speed":{"value":12.0}}},{"time":1574376583000,"data":{"speed":{"value":13.0}}},{"time":1574376584000,"data":{"speed":{"value":14.0}}},{"time":1574376585000,"data":{"speed":{"value":14.0}}},{"time":1574376586000,"data":{"speed":{"value":14.0}}},{"time":1574376587000,"data":{"location":{"lat":-12.062012,"lng":-77.096057,"hDop":0,"satNum":7},"speed":{"value":13.0}}},{"time":1574376588000,"data":{"speed":{"value":12.0}}},{"time":1574376589000,"data":{"speed":{"value":11.0}}},{"time":1574376590000,"data":{"speed":{"value":11.0}}},{"time":1574376591000,"data":{"speed":{"value":7.0}}},{"time":1574376592000,"data":{"location":{"lat":-12.062148,"lng":-77.096038,"hDop":0,"satNum":7},"speed":{"value":5.0}}},{"time":1574376593000,"data":{"speed":{"value":2.0}}},{"time":1574376594000,"data":{"speed":{"value":1.0}}},{"time":1574376595000,"data":{"speed":{"value":0.0}}},{"time":1574376596000,"data":{"speed":{"value":0.0}}},{"time":1574376597000,"data":{"location":{"lat":-12.062167,"lng":-77.096038,"hDop":0,"satNum":7},"speed":{"value":0.0}}},{"time":1574376598000,"data":{"speed":{"value":0.0}}},{"time":1574376599000,"data":{"speed":{"value":0.0}}},{"time":1574376600000,"data":{"speed":{"value":0.0}}},{"time":1574376601000,"data":{"speed":{"value":0.0}}},{"time":1574376602000,"data":{"speed":{"value":0.0},"location":{"lat":-12.062167,"lng":-77.096038,"hDop":0,"satNum":7}}},{"time":1574376603000,"data":{"speed":{"value":0.0}}},{"time":1574376604000,"data":{"speed":{"value":0.0}}},{"time":1574376605000,"data":{"speed":{"value":0.0}}},{"time":1574376606000,"data":{"speed":{"value":0.0}}},{"time":1574376607000,"data":{"location":{"lat":-12.062173,"lng":-77.096045,"hDop":0,"satNum":7},"speed":{"value":0.0}}},{"time":1574376608000,"data":{"speed":{"value":0.0}}},{"time":1574376609000,"data":{"speed":{"value":0.0}}},{"time":1574376610000,"data":{"speed":{"value":0.0}}},{"time":1574376611000,"data":{"speed":{"value":0.0}}},{"time":1574376612000,"data":{"location":{"lat":-12.062175,"lng":-77.096052,"hDop":0,"satNum":7},"speed":{"value":0.0}}},{"time":1574376613000,"data":{"speed":{"value":0.0}}},{"time":1574376614000,"data":{"speed":{"value":0.0}}},{"time":1574376615000,"data":{"speed":{"value":0.0}}},{"time":1574376616000,"data":{"speed":{"value":0.0}}},{"time":1574376617000,"data":{"speed":{"value":0.0},"location":{"lat":-12.062175,"lng":-77.096052,"hDop":0,"satNum":7}}},{"time":1574376618000,"data":{"speed":{"value":0.0}}},{"time":1574376619000,"data":{"speed":{"value":0.0}}},{"time":1574376620000,"data":{"speed":{"value":0.0}}},{"time":1574376621000,"data":{"speed":{"value":0.0}}},{"time":1574376622000,"data":{"location":{"lat":-12.062175,"lng":-77.096058,"hDop":0,"satNum":7},"speed":{"value":0.0}}},{"time":1574376623000,"data":{"speed":{"value":1.0}}},{"time":1574376624000,"data":{"speed":{"value":1.0}}},{"time":1574376625000,"data":{"speed":{"value":1.0}}},{"time":1574376626000,"data":{"speed":{"value":1.0}}},{"time":1574376627000,"data":{"location":{"lat":-12.062175,"lng":-77.096058,"hDop":0,"satNum":7},"speed":{"value":1.0}}},{"time":1574376628000,"data":{"speed":{"value":2.0}}},{"time":1574376629000,"data":{"speed":{"value":3.0}}},{"time":1574376630000,"data":{"speed":{"value":2.0}}},{"time":1574376631000,"data":{"speed":{"value":1.0}}},{"time":1574376632000,"data":{"speed":{"value":0.0},"location":{"lat":-12.062153,"lng":-77.09606,"hDop":0,"satNum":7}}},{"time":1574376633000,"data":{"speed":{"value":0.0}}},{"time":1574376634000,"data":{"speed":{"value":0.0}}},{"time":1574376635000,"data":{"speed":{"value":0.0}}},{"time":1574376636000,"data":{"speed":{"value":0.0}}},{"time":1574376640000,"data":{"heartBeat":{"count":1},"tripEnd":{"occur":true,"timezone":4.0,"odometer":567.8},"tripSummary":{"consumedFuel":123.4,"distance":24.2}}}],"tags":[]}"#;
    let data_package_v2 = serde_json::from_str::<DataPackageV2>(data_package_v2_str).unwrap();

    let header = HeaderNotification{
        created: 12345678,
        metadata: Value::Null,
        microservice: "test_service".to_string()
    };

    let result = event_notificator
                    .notify(&header, &data_package_v2)
                    .await
                    .unwrap();
}
```