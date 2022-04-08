use std::{fs::File, io::Read};

use dotenvy::dotenv;
use jwt_simple::prelude::RS384KeyPair;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub redis_urls: String,
    pub server: String,
    pub jwt_rsa: String,
    pub jwt_expiration: i64,
}

impl Config {
    pub fn get_rsa(&self) -> RS384KeyPair {
         let mut rsa_file = File::open(&self.jwt_rsa).unwrap_or_else(|_| panic!("failed to load private key file:{}",&self.jwt_rsa));
         let mut contents = String::new();
         rsa_file.read_to_string(&mut contents).unwrap();
         RS384KeyPair::from_pem(contents.as_str()).expect("failed to load private key")
    }
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn get_config() -> Config {
    dotenv().ok();
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error:{:#?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_a_config() {
        let config = get_config();
        assert_ne!(config.server, "".to_string());
    }
    #[test]
    fn get_config_from_the_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.server, "".to_string());
    }
}
