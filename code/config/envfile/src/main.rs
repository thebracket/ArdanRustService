fn main() {
    // Ignore the result of loading .env --- it's ok if it doesn't exist
    let _ = dotenvy::dotenv();

    // Obtain the contents of the TESTVAR environment variable
    let testvar = std::env::var("TESTVAR").unwrap_or_else(|_| "default".to_string());

    // Print it
    println!("{testvar}");
}
