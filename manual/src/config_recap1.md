# Configuration Recap

So far, we've:

* Used the `dotenvy` crate to load environment variables from `.env` files. This is optional, but really useful for configuring test environments.
* Used `Config` to load configuration from environment variables, files and network sources.
* Used `clap` to handle CLI configuration of the service.

That's all the building blocks you need. We'll revisit this a bit when we get to the service design section.
