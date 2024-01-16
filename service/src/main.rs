use service_framework::{run_services, run_webservices, Service};
use webservice::{LibrarianService, MyService, OtherService};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    run_services!("0.0.0.0:3001", MyService, OtherService, LibrarianService);
    Ok(())
}
