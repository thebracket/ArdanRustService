# Environment Variables with .env

Let's start with `.env` support. In Rust Foundations and other videos, we frequently used a `.env` file to store environment variables---and used the `dotnev` crate to load them. This remains a good practice, even if you won't use it on production; it provides a quick and easy way to tweak your setup, especially during development.

> The code for this may be found in `code/config/envfile`.

Let's start a new project:

```bash
cargo new envfile
cd envfile
cargo add dotenvy
```

> Note: we're using `dotenvy` because it has superceded `dotenv` in a lot of projects.

Now let's make a program that reads a `.env` file, and outputs the contents of the `TESTVAR` environment variable:

```rust
fn main() {
    // Ignore the result of loading .env --- it's ok if it doesn't exist
    let _ = dotenvy::dotenv();

    // Obtain the contents of the TESTVAR environment variable
    let testvar = std::env::var("TESTVAR").unwrap_or_else(|_| "default".to_string());

    // Print it
    println!("{testvar}");
}
```

Now run this:

```bash
cargo run
default
TESTVAR=test cargo run
test
```

You can create a file named `.env` in the project's root and set the variable there:

```
TESTVAR=test
```

Now you can `cargo run` and see the desired `test` value.

That's not amazing, but it's a good start. You can pass environment variables into your program to use in configuration.