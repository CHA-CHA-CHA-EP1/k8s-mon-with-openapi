use serde::Deserialize;

#[derive(Deserialize ,Debug)]
pub struct Config {
    pub namespace: String,
}

impl Config {
    pub fn default() -> Result<Config, envy::Error> {
        envy::from_env::<Config>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config {
            namespace: "default".to_string(),
        };
        assert_eq!(config.namespace, "default");
    }
}
