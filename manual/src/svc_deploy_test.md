# Build a Test Service

We'll quickly put together a test service for deployment.

Go through the code - we'll look at it in detail later. Notice:
* It loads configuration from environment variables.
* We have `.env` set to mock the environment variables for testing.
* There's an `auth` and `bookstore` service.
    * Both have their own configuration.
    * Both have their own databases. We used a "new type" to differentiate between the database pools.
    * Each has a secure section using a layer for token-based security.
* The `static_html` directory is an HTML+Javascript application.

The important part is that the executable is self-contained, other than *also* requiring a `static_html` directory.