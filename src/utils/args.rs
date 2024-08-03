use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about="The official Neutron websocket connection server that is in charge of transporting and managing network interactions for individual robot connections", long_about = None)]
pub struct Args {
    #[clap(long)]
    /// The identifier of the connection
    pub id: String,

    #[clap(long, short, short = 'c')]
    /// The hostname of the robot to be connected to
    pub robot_host: String,

    #[clap(long)]
    /// The port of the robot to be connected to its context (RosBridge, Neutron...)
    pub robot_context_port: u16,

    #[clap(long)]
    /// The port of the robot to be connected to the neutron agent
    pub robot_agent_port: u16,

    #[clap(long, short, short = 'p')]
    /// The port for the application to run on
    pub application_port: u16,

    #[clap(long, short, short = 't')]
    /// The optional timeout (in seconds) before closing the app if no client are connected
    pub application_timeout: Option<u64>,

    #[clap(long, short, short = 'r')]
    /// The optional redis connection string for creating the db client
    pub redis_connection_string: Option<String>,
}

pub fn print_args(args: &Args) {
    info!(target: "init", "Intializing the neutron connection with following arguments:");
    info!(target: "init", "id: {}", args.id);
    info!(target: "init", "robot_host: {}", args.robot_host);
    info!(target: "init", "robot_agent_port: {}", args.robot_agent_port);
    info!(target: "init", "robot_context_port: {}", args.robot_context_port);
    info!(target: "init", "application_port: {}", args.application_port);
    if let Some(redis_connection_string) = &args.redis_connection_string {
        info!(target: "init", "redis_connection_string: {}", redis_connection_string);
    }
}
