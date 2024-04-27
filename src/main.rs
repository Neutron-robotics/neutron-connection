pub mod network;
use chrono::Local;
use clap::Parser;
use database::redis::make_redis_connection;
use env_logger::Builder;
use log::{error, info, warn, LevelFilter};
use std::io::Write;
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
    let connection_id = args.id.clone();

    let mut builder = Builder::from_default_env();
    builder
        .format(move |buf, record| {
            writeln!(
                buf,
                "[{}][{}][{}][{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
                record.level(),
                record.target(),
                connection_id,
                record.args()
            )
        })
        .target(env_logger::Target::Stdout)
        .filter(None, LevelFilter::Info)
        .init();

    // env_logger::init_from_env(
    //     env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    // );

    let shared_connection_context = SharedConnectionContext::default();

    shared_connection_context.write().await.application_timeout = args.application_timeout;
    shared_connection_context.write().await.id = args.id.clone();
    shared_connection_context.write().await.robot_hostname = args.robot_host.clone();
    shared_connection_context.write().await.robot_port = args.robot_port;

    utils::args::print_args(&args);

    match args.redis_connection_string {
        Some(redis_connection_string) => {
            info!(target: "init", "Using redis connection string {}", redis_connection_string);
            let conn = make_redis_connection(&redis_connection_string);
            shared_connection_context.write().await.redis_connection = Some(conn);
        }
        None => {
            warn!(target: "init", "No Redis connection will be established for this connection");
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
            info!(target: "init",
                "Robot client connected at address ws://{}:{}",
                &args.robot_host, &args.robot_port
            );
            info!(target: "init",
                "neutron connection {} ready",
                shared_connection_context.write().await.id
            );
        }
        Err(err) => {
            error!(target: "init", "Robot client encountered an error: {}", err)
        }
    }

    server_start(args.application_port, shared_connection_context).await;
}
