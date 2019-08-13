extern crate event_notificator;

use std::process;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

use event_notificator::notificator_config::NotificatorConfig;
use event_notificator::notificator_builder::NotificatorBuilder;


const REQUIRED_EXT: &str = "toml";
const MISSING_CONFIG_FILE: &str = "You must provide a *.toml config file";
const CONFIG_FILE_NOTFOUND: &str = "Provided config file not found";
const ERROR_LOADING_CONFIG: &str = "Error loading config file:";

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.get(1).is_none() {
        println!("{}", MISSING_CONFIG_FILE);
        process::exit(1);
    }

    let config_file_name = &args[1];
    let config_file_path = Path::new(config_file_name);

    if config_file_path.extension().and_then(OsStr::to_str) != Some(REQUIRED_EXT) {
        println!("{}", MISSING_CONFIG_FILE);
        process::exit(1);
    }

    if !config_file_path.exists() {
        println!("{}", CONFIG_FILE_NOTFOUND);
        process::exit(1);
    }

    let config = match NotificatorConfig::load(config_file_path) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", ERROR_LOADING_CONFIG);
            println!("{:?}", e.to_string());
            process::exit(1);
        }
    };

    let _event_notificator = NotificatorBuilder::from_config(config);
}