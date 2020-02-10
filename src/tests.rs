#[cfg(test)]
mod tests {
    use mockito::{ mock };

    use crate::notificator_config::NotificatorStrategiesConfig;
    use crate::notificator_builder::NotificatorBuilder;
    use crate::notifications::data_package_notification::HeaderNotification;
    use agni_client::client::AgniClientConfig;
    use serde_json;
    use data_package_v2::data_package_v2::DataPackageV2;
    use serde_json::Value;
    use crate::event_notificator::EventNotificator;

    const INBOX: &str = "event_notificator_test";

    fn init_notificator(agni: String) -> EventNotificator{
        let config = NotificatorStrategiesConfig::AgniNotificator {
            topic: INBOX.to_string(),
            environments: [("production-01".to_string(), INBOX.to_string())].iter().cloned().collect(),
            agni_config: AgniClientConfig::new(agni, "event_notificator".to_string())
        };

        NotificatorBuilder::from_strategies_config(config)
    }

    #[test]
    fn notify() {
        // Pre-conditions
        let _m1 = mock("POST", format!("/api/topics/{}/publish", INBOX).as_str())
            .with_status(201)
            .with_body(r#"{
                "status": "ok"
            }"#)
            .create();

        let event_notificator = init_notificator(mockito::server_url().clone());

        // Test setup
        let data_package_v2_str = r#"{"id":"FAIFEDY2amM5dQGepxM9HEQBACwB","type":"tripCompleted","created":1579624709812,"dataSource":{"type":"imei","value":"356363051927467"},"dataPoints":[{"time":1579547243000,"data":{"generatedTripStart":{"type":"generatedTripStart","reason":"The device is stopped and a new package arrived without TripStart event"},"speed":{"type":"speed","value":22},"location":{"type":"location","lat":-33.392783,"lng":-70.533437,"hDop":0,"satNum":7}}},{"time":1579547245000,"data":{"speed":{"type":"speed","value":29}}},{"time":1579547247000,"data":{"speed":{"type":"speed","value":37}}},{"time":1579547248000,"data":{"acceleration":{"type":"acceleration","x":0.203125,"y":-0.140625,"z":1},"location":{"type":"location","lat":-33.393083,"lng":-70.533182,"hDop":0,"satNum":7}}},{"time":1579547248250,"data":{"acceleration":{"type":"acceleration","x":0.21875,"y":-0.109375,"z":0.984375}}},{"time":1579547248500,"data":{"acceleration":{"type":"acceleration","x":0.203125,"y":-0.109375,"z":0.953125}}},{"time":1579547248750,"data":{"acceleration":{"type":"acceleration","x":0.203125,"y":-0.109375,"z":0.921875}}},{"time":1579547249000,"data":{"speed":{"type":"speed","value":43}}},{"time":1579547250000,"data":{"speed":{"type":"speed","value":49},"acceleration":{"type":"acceleration","x":0.171875,"y":-0.0625,"z":1.125}}},{"time":1579547250250,"data":{"acceleration":{"type":"acceleration","x":0.171875,"y":-0.03125,"z":1}}},{"time":1579547250500,"data":{"acceleration":{"type":"acceleration","x":0.1875,"y":-0.0625,"z":1}}},{"time":1579547250750,"data":{"acceleration":{"type":"acceleration","x":0.203125,"y":-0.0625,"z":0.9375}}},{"time":1579547252000,"data":{"location":{"type":"location","lat":-33.393653,"lng":-70.533103,"hDop":0,"satNum":7},"acceleration":{"type":"acceleration","x":0.1875,"y":-0.140625,"z":1.046875},"speed":{"type":"speed","value":53}}},{"time":1579547252250,"data":{"acceleration":{"type":"acceleration","x":0.1875,"y":-0.140625,"z":1.03125}}},{"time":1579547252500,"data":{"acceleration":{"type":"acceleration","x":0.25,"y":-0.125,"z":1.015625}}},{"time":1579547252750,"data":{"acceleration":{"type":"acceleration","x":0.1875,"y":-0.1875,"z":0.984375}}},{"time":1579547254000,"data":{"speed":{"type":"speed","value":57}}},{"time":1579547256000,"data":{"speed":{"type":"speed","value":59}}},{"time":1579547257000,"data":{"location":{"type":"location","lat":-33.394335,"lng":-70.533362,"hDop":0,"satNum":7}}},{"time":1579547258000,"data":{"speed":{"type":"speed","value":59}}},{"time":1579547260000,"data":{"speed":{"type":"speed","value":55}}},{"time":1579547262000,"data":{"acceleration":{"type":"acceleration","x":-0.1875,"y":0.0625,"z":0.921875},"location":{"type":"location","lat":-33.394993,"lng":-70.533617,"hDop":0,"satNum":7},"speed":{"type":"speed","value":42}}},{"time":1579547262250,"data":{"acceleration":{"type":"acceleration","x":-0.296875,"y":0.078125,"z":1.015625}}},{"time":1579547262500,"data":{"acceleration":{"type":"acceleration","x":-0.234375,"y":0.0625,"z":1.046875}}},{"time":1579547262750,"data":{"acceleration":{"type":"acceleration","x":-0.234375,"y":0.03125,"z":0.984375}}},{"time":1579547263000,"data":{"speed":{"type":"speed","value":26}}},{"time":1579547264000,"data":{"acceleration":{"type":"acceleration","x":-0.21875,"y":0.046875,"z":1}}},{"time":1579547264250,"data":{"acceleration":{"type":"acceleration","x":-0.171875,"y":0.046875,"z":1.03125}}},{"time":1579547264500,"data":{"acceleration":{"type":"acceleration","x":-0.109375,"y":0.046875,"z":1}}},{"time":1579547264750,"data":{"acceleration":{"type":"acceleration","x":-0.03125,"y":0.046875,"z":1}}},{"time":1579547265000,"data":{"acceleration":{"type":"acceleration","x":0,"y":0.046875,"z":0.953125},"speed":{"type":"speed","value":20}}},{"time":1579547265250,"data":{"acceleration":{"type":"acceleration","x":-0.03125,"y":0.0625,"z":0.984375}}},{"time":1579547265500,"data":{"acceleration":{"type":"acceleration","x":-0.09375,"y":0.03125,"z":1.34375}}},{"time":1579547265750,"data":{"acceleration":{"type":"acceleration","x":0.03125,"y":0.0625,"z":0.671875}}},{"time":1579547266000,"data":{"acceleration":{"type":"acceleration","x":0.203125,"y":0.109375,"z":0.96875},"location":{"type":"location","lat":-33.39532,"lng":-70.533742,"hDop":0,"satNum":7}}},{"time":1579547266250,"data":{"acceleration":{"type":"acceleration","x":0.265625,"y":0.0625,"z":0.96875}}},{"time":1579547266500,"data":{"acceleration":{"type":"acceleration","x":0.265625,"y":0.09375,"z":0.96875}}},{"time":1579547266750,"data":{"acceleration":{"type":"acceleration","x":0.1875,"y":0.0625,"z":1.015625}}},{"time":1579547267000,"data":{"speed":{"type":"speed","value":29}}},{"time":1579547268000,"data":{"speed":{"type":"speed","value":37},"acceleration":{"type":"acceleration","x":0.265625,"y":0.0625,"z":0.953125}}},{"time":1579547268250,"data":{"acceleration":{"type":"acceleration","x":0.171875,"y":0.09375,"z":1.03125}}},{"time":1579547268500,"data":{"acceleration":{"type":"acceleration","x":0.21875,"y":0.078125,"z":0.96875}}},{"time":1579547268750,"data":{"acceleration":{"type":"acceleration","x":0.125,"y":0.0625,"z":0.9375}}},{"time":1579547269000,"data":{"location":{"type":"location","lat":-33.395725,"lng":-70.533925,"hDop":0,"satNum":7},"speed":{"type":"speed","value":45}}},{"time":1579547270000,"data":{"acceleration":{"type":"acceleration","x":0.125,"y":0.09375,"z":0.953125}}},{"time":1579547270250,"data":{"acceleration":{"type":"acceleration","x":0.234375,"y":0.03125,"z":0.953125}}},{"time":1579547270500,"data":{"acceleration":{"type":"acceleration","x":0.15625,"y":0.0625,"z":1.015625}}},{"time":1579547270750,"data":{"acceleration":{"type":"acceleration","x":0.1875,"y":0.046875,"z":0.96875}}},{"time":1579547271000,"data":{"acceleration":{"type":"acceleration","x":0.140625,"y":0.09375,"z":1.015625},"speed":{"type":"speed","value":52}}},{"time":1579547271250,"data":{"acceleration":{"type":"acceleration","x":0.109375,"y":0.09375,"z":1}}},{"time":1579547271500,"data":{"acceleration":{"type":"acceleration","x":0.15625,"y":0.140625,"z":0.96875}}},{"time":1579547271750,"data":{"acceleration":{"type":"acceleration","x":0.171875,"y":0.078125,"z":0.953125}}},{"time":1579547273000,"data":{"speed":{"type":"speed","value":57}}},{"time":1579547502000,"data":{"acceleration":{"type":"acceleration","x":0.171875,"y":-0.03125,"z":0.984375}}},{"time":1579547502250,"data":{"acceleration":{"type":"acceleration","x":0.203125,"y":-0.078125,"z":0.984375}}},{"time":1579547502500,"data":{"acceleration":{"type":"acceleration","x":0.203125,"y":-0.046875,"z":1}}},{"time":1579547502750,"data":{"acceleration":{"type":"acceleration","x":0.1875,"y":-0.0625,"z":1.015625}}},{"time":1579547503000,"data":{"acceleration":{"type":"acceleration","x":0.1875,"y":-0.078125,"z":1.109375}}},{"time":1579547503250,"data":{"acceleration":{"type":"acceleration","x":0.21875,"y":-0.125,"z":1.03125}}},{"time":1579547503500,"data":{"acceleration":{"type":"acceleration","x":0.265625,"y":-0.1875,"z":0.875}}},{"time":1579547503750,"data":{"acceleration":{"type":"acceleration","x":0.1875,"y":-0.125,"z":1}}},{"time":1579547516000,"data":{"acceleration":{"type":"acceleration","x":-0.015625,"y":0.03125,"z":1.015625}}},{"time":1579547516250,"data":{"acceleration":{"type":"acceleration","x":-0.0625,"y":0.078125,"z":1.046875}}},{"time":1579547516500,"data":{"acceleration":{"type":"acceleration","x":-0.0625,"y":0.0625,"z":1.03125}}},{"time":1579547516750,"data":{"acceleration":{"type":"acceleration","x":-0.03125,"y":0.03125,"z":0.953125}}},{"time":1579547517000,"data":{"acceleration":{"type":"acceleration","x":-0.03125,"y":0.046875,"z":0.984375}}},{"time":1579547517250,"data":{"acceleration":{"type":"acceleration","x":-0.046875,"y":0.046875,"z":0.96875}}},{"time":1579547517500,"data":{"acceleration":{"type":"acceleration","x":-0.078125,"y":0.078125,"z":0.953125}}},{"time":1579547517750,"data":{"acceleration":{"type":"acceleration","x":-0.1875,"y":0.078125,"z":1.03125}}},{"time":1579547520000,"data":{"acceleration":{"type":"acceleration","x":-0.109375,"y":0.09375,"z":1.3125}}},{"time":1579547520250,"data":{"acceleration":{"type":"acceleration","x":-0.03125,"y":0.015625,"z":1.078125}}},{"time":1579547520500,"data":{"acceleration":{"type":"acceleration","x":0.046875,"y":0.078125,"z":0.8125}}},{"time":1579547520750,"data":{"acceleration":{"type":"acceleration","x":0.078125,"y":0.046875,"z":1.015625}}},{"time":1579547521000,"data":{"acceleration":{"type":"acceleration","x":0.140625,"y":0.046875,"z":0.9375}}},{"time":1579547521250,"data":{"acceleration":{"type":"acceleration","x":0.15625,"y":0.046875,"z":0.984375}}},{"time":1579547521500,"data":{"acceleration":{"type":"acceleration","x":0.171875,"y":0.046875,"z":1.015625}}},{"time":1579547521750,"data":{"acceleration":{"type":"acceleration","x":0.203125,"y":0.078125,"z":0.984375},"generatedTripEnd":{"type":"generatedTripEnd","reason":"Forced Finish, 600s without received data","forced":true}}}],"tags":[{"environment":"production-01","keys":{"tenants":["5a04e8384f89c74080bd2d72","5a04e8384f89c74080bd2asd"],"user":"5c53a7f3f4b5ce7771493abe","device":"5a625df88c62567dac0fee0f","vehicle":"5c53a7eeafaf8c3878b05a49","imei":"356363051927467"}}],"metadata":{"timezones":{"type":"timezones","values":[{"start":1579547243000,"end":1579547521750,"name":"Migration","offset":-10800000}]},"source":{"type":"source","value":"DanlawGW"},"anomaly":{"detected":false,"reason":"","status":0,"type":"anomaly"},"business":{"billingPeriod":["2020-01"],"type":"business"}},"aggregations":{"end":{"type":"end","lat":-33.395725,"lng":-70.533925,"time":1579547521750,"autoGenerated":true,"forced":true},"start":{"type":"start","lat":-33.392783,"lng":-70.533437,"time":1579547243000,"autoGenerated":true},"monthPeriod":{"type":"monthPeriod","periods":["2019-12","2020-01"]},"weekPeriod":{"type":"weekPeriod","periods":["2019-52","2020-01"]}},"accumulations":{"idleTime":{"type":"idleTime","duration":0,"units":"millis"},"averageSpeed":{"type":"averageSpeed","total":771,"readings":18,"avg":42.833333333333336,"units":"km/h"},"distance":{"type":"distance","last_data_point":{"time":1579547273000,"data":{"location":{"type":"location","lat":-33.395725,"lng":-70.533925,"hDop":0,"satNum":7},"speed":{"type":"speed","value":57}}},"value":0.3636111111111111,"units":"km","score":0.9,"method":"speed"},"maximumSpeed":{"type":"maximumSpeed","value":59,"units":"km/h"},"duration":{"type":"duration","start":1579547243000,"end":1579547521750,"value":278750,"units":"millis"},"accelerations":{"type":"accelerations","histogram":[{"value":0.1,"count":15},{"value":0.2,"count":12},{"value":0.3,"count":9},{"value":0.4,"count":5},{"value":0.8,"count":2},{"value":1.0,"count":1},{"value":-0.2,"count":9},{"value":-0.3,"count":7},{"value":-0.4,"count":5},{"value":-0.8,"count":2},{"value":-1.0,"count":1}]},"turns":{"type":"turns","histogram":[{"value":0.1,"count":15},{"value":0.2,"count":10},{"value":0.3,"count":9},{"value":0.4,"count":7},{"value":0.8,"count":3},{"value":1.0,"count":1},{"value":-0.2,"count":12},{"value":-0.3,"count":9},{"value":-0.4,"count":7},{"value":-0.8,"count":3},{"value":-1.1,"count":1}]}}}"#;
        let data_package_v2 = serde_json::from_str::<DataPackageV2>(data_package_v2_str).unwrap();

        let header = HeaderNotification{
            created: 12345678,
            metadata: Value::Null,
            microservice: "test_service".to_string()
        };

        // Test
        let response = event_notificator.notify(&header, &data_package_v2).unwrap();

        assert_eq!((), response)
    }
}