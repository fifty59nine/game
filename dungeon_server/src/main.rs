mod logic;

use clap::Parser;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = ServerParams::parse();
    let addr: String = format!("{}:{}", &args.host, &args.port);
    let game = logic::Game::new(addr);
    let server = game.get_server();
    server.run_server(Arc::new(Mutex::new(game))).await?;
    Ok(())
}

#[derive(Debug, Parser)]
pub struct ServerParams {
    /// IP without port. Default: 0.0.0.0
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    /// Server port. Default 7777
    #[arg(short, default_value_t = String::from("7777"))]
    port: String,
}
