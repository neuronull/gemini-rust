use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Api {
    pub url: String,
    pub key: String,
    pub sec: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Cfg {
    pub api: Api,
}

impl Cfg {
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::new();
        c.merge(File::with_name("tests/cfg")).unwrap();
        c.try_into()
    }
}
