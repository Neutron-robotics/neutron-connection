pub mod network;
use clap::Parser;
use database::redis::get_connection;
use utils::args::Args;
pub mod database;
pub mod utils;
use network::{connection_context::SharedConnectionContext, ws_proxy::server_start, ws_client::websocket_client};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let conn = get_connection(&args.redis_connection_string);
    utils::args::print_args(&args);
    let shared_connection_context = SharedConnectionContext::default();
    shared_connection_context.write().await.redis_connection = Some(conn);

    match websocket_client(&shared_connection_context, &args.robot_host, &args.robot_port).await {
        Ok(()) => {
            println!("Robot client connected at address ws://{}:{}", &args.robot_host, &args.robot_port)
        }
        Err(err) => {
            eprintln!("Robot client encountered an error: {}", err);
        }
    }

    server_start(args.application_port, shared_connection_context).await;
}
