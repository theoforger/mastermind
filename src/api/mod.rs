use crate::configs::config::Config;
use dotenv::dotenv;
use std::env;

mod chat_completions;
mod models;

pub struct Instance {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl Instance {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::new()?;
        dotenv().ok();

        let base_url =
            Self::read_from_env_or_config_file("OPENAI_API_BASE_URL", config.get_base_url())?;

        let base_url = if !base_url.ends_with('/') {
            format!("{base_url}/")
        } else {
            base_url
        };

        let api_key = Self::read_from_env_or_config_file("API_KEY", config.get_api_key())?;

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
            api_key,
        })
    }

    fn read_from_env_or_config_file(
        envvar: &str,
        config_value: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match env::var(envvar) {
            Ok(key) => Ok(key),
            Err(_) => {
                if let Some(config_key) = config_value {
                    Ok(config_key.to_string())
                } else {
                    Err(format!(
                        "Could not find environment variable '{envvar}' or any related configuration\nPlease check you config file"
                    )
                        .into())
                }
            }
        }
    }
}

#[cfg(test)]
impl Instance {
    pub(crate) fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}

#[cfg(test)]
impl Default for Instance {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "".to_string(),
            api_key: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        env::set_var("OPENAI_API_BASE_URL", "abc");
        env::set_var("API_KEY", "def");

        let api_instance = Instance::new().unwrap();
        assert_eq!(api_instance.base_url, "abc/");
        assert_eq!(api_instance.api_key, "def");
    }

    #[test]
    fn test_default() {
        let api_instance = Instance::default();
        assert_eq!(api_instance.base_url, "");
        assert_eq!(api_instance.api_key, "");
    }
}
