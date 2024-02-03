pub mod network;
use clap::Parser;
use database::redis::make_redis_connection;
use utils::args::Args;
pub mod database;
pub mod utils;
use network::{
    connection_context::SharedConnectionContext, ws_client::websocket_client,
    ws_proxy::server_start,
};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let shared_connection_context = SharedConnectionContext::default();

    utils::args::print_args(&args);

    match args.redis_connection_string {
        Some(redis_connection_string) => {
            println!("Using redis connection string {}", redis_connection_string);
            let conn = make_redis_connection(&redis_connection_string);
            shared_connection_context.write().await.redis_connection = Some(conn);
        }
        None => {
            println!("No Redis connection will be established for this connection")
        }
    }

    match websocket_client(
        &shared_connection_context,
        &args.robot_host,
        &args.robot_port,
    )
    .await
    {
        Ok(()) => {
            println!(
                "Robot client connected at address ws://{}:{}",
                &args.robot_host, &args.robot_port
            );
            println!("neutron connection {} ready", args.id);
        }
        Err(err) => {
            eprintln!("Robot client encountered an error: {}", err);
        }
    }

    server_start(args.application_port, shared_connection_context).await;
}
