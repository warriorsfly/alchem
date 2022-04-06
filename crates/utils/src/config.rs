use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server: String,
    pub private_key_path: String,
    pub jwt_expiration: i64,
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
