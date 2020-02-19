#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate failure;
extern crate async_trait;

extern crate agni_client;
extern crate futures;
extern crate data_package_v2;

pub mod notificator_strategy;
pub mod event_notificator;
pub mod notificator_config;
pub mod notificator_builder;
pub mod notifications;
pub mod strategies;

pub mod tests;
