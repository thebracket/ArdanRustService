# Native Host Deployment

Deploying natively is very easy:

1. Build the site in `release` mode.
    * `cargo build --release`
2. Copy `deploy_bookstore` to your target folder.
3. Copy the `static_html` directory into the target folder.

You can now run `./deploy_bookstore`. On Linux, you can use `systemd` to make it into a service if you wish.

You don't need any dependencies or runtimes: all dependencies are baked into the executable.

Native deployment is also the highest performing option. The same steps apply to "native" inside a VM.