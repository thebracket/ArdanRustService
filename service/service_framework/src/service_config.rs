use anyhow::Result;
use config::Config;
use serde::de::DeserializeOwned;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use tracing::debug;

static DOT_ENV_LOADED: AtomicBool = AtomicBool::new(false);

pub enum ConfigFileType {
    None,
    File { filename: String, required: bool },
}

pub fn load_config<T>(env_prefix: &str, config_file: ConfigFileType) -> Result<Arc<T>>
where
    T: DeserializeOwned,
{
    if !DOT_ENV_LOADED.load(Relaxed) {
        // Load a ".env" file if it exists
        debug!("Loading environment from .env");
        if dotenv::dotenv().is_err() {
            debug!("Loading .env file failed.")
        };
        DOT_ENV_LOADED.store(true, Relaxed);
    }

    // Setup the configuration file based on preferences
    let settings = match config_file {
        ConfigFileType::None => Config::builder()
            .add_source(config::Environment::with_prefix(env_prefix))
            .build()?,
        ConfigFileType::File { filename, required } => Config::builder()
            .add_source(config::File::with_name(&filename).required(required))
            .add_source(config::Environment::with_prefix(env_prefix))
            .build()?,
    };

    // Try to deserialize into the requested configuration format
    let result = settings.try_deserialize::<T>()?;

    Ok(Arc::new(result))
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Deserialize;
    use std::fs::{remove_file, File};
    use std::io::Write;

    #[derive(Deserialize)]
    struct ConfigTest {
        some_text: String,
    }

    #[test]
    fn test_env_only() {
        std::env::set_var("TEST_SERVICE_SOME_TEXT", "Hello");
        let settings = load_config::<ConfigTest>("TEST_SERVICE", ConfigFileType::None).unwrap();
        std::env::remove_var("TEST_SERVICE_SOME_TEXT");

        assert_eq!(settings.some_text, "Hello");
    }

    #[test]
    fn test_file() {
        let mut source = std::env::temp_dir();
        source.push("test_config.json");
        let mut file = File::create(&source).unwrap();
        file.write_all(b"{ \"some_text\" : \"Hello\" }").unwrap();

        let settings = load_config::<ConfigTest>(
            "TEST_SERVICE",
            ConfigFileType::File {
                filename: source.to_string_lossy().to_string(),
                required: true,
            },
        )
        .unwrap();
        assert_eq!(settings.some_text, "Hello");

        remove_file(&source).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_required_file_absent() {
        let mut source = std::env::temp_dir();
        source.push("test_config_missing.json");

        let _settings = load_config::<ConfigTest>(
            "TEST_SERVICE",
            ConfigFileType::File {
                filename: source.to_string_lossy().to_string(),
                required: true,
            },
        )
        .unwrap();
    }
}
