use axum::{response::Html, routing::get, Router};
use clap::{value_parser, Arg, Command};

#[tokio::main]
async fn main() {
    let matches = Command::new("simple_http_server")
        .version("0.1.0")
        .author("Herbert")
        .subcommand(
            Command::new("serve")
                .about("Starts the server")
                .arg(
                    Arg::new("address")
                        .short('a')
                        .long("address")
                        .value_name("ADDRESS")
                        .help("Sets the IP address to bind to"),
                )
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .value_name("PORT")
                        .help("Sets the port to bind to")
                        .value_parser(value_parser!(u16)),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("serve") {
        let address: String = matches
            .get_one("address")
            .cloned()
            .unwrap_or("127.0.0.1".to_string());
        let port: u16 = *matches.get_one("port").unwrap_or(&3001);
        let bind_address = format!("{}:{}", address, port);
        serve(&bind_address).await;
    } else {
        println!("Run with --help for details");
    }
}

async fn serve(bind_address: &str) {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
