#[macro_use]
extern crate serde_derive;
extern crate async_trait;
extern crate serde;
extern crate serde_json;
extern crate thiserror;

extern crate agni_client;
extern crate data_package_v2;

pub mod errors;
pub mod event_notificator;
pub mod notifications;
pub mod notificator_builder;
pub mod notificator_config;
pub mod notificator_strategy;
pub mod strategies;
