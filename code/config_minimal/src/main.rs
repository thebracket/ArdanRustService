use std::collections::HashMap;
use config::Config;

fn main() {
    // Ignore the result of loading .env --- it's ok if it doesn't exist
    let _ = dotenvy::dotenv();

    let settings_reader = Config::builder()
        .add_source(config::File::with_name("settings").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let settings = settings_reader
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    println!("{settings:?}");
}
