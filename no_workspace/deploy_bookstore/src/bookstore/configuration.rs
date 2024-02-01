use anyhow::Result;
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookstoreConfiguration {
    pub db_filename: String,
}

impl BookstoreConfiguration {
    pub fn load() -> Result<Self> {
        // Load any .env files
        // Ignore the result of loading .env --- it's ok if it doesn't exist
        let _ = dotenvy::dotenv();

        let settings_reader = Config::builder()
            .add_source(config::File::with_name("settings").required(false))
            .add_source(config::Environment::with_prefix("BOOKSTORE"))
            .build()?;

        let settings = settings_reader
            .try_deserialize()?;

        Ok(settings)

    }
}