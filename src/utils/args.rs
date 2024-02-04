use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about="The official Neutron websocket connection server that is in charge of transporting and managing network interactions for individual robot connections", long_about = None)]
pub struct Args {
    #[clap(long)]
    /// The identifier of the connection
    pub id: String,

    #[clap(long, short, short = 'c')]
    /// The hostname of the robot to be connected to
    pub robot_host: String,

    #[clap(long, short, short = 'd')]
    /// The port of the robot to be connected to
    pub robot_port: u16,

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
    println!("Args:");
    println!("id: {}", args.id);
    println!("robot_host: {}", args.robot_host);
    println!("robot_port: {}", args.robot_port);
    println!("application_port: {}", args.application_port);
    match &args.redis_connection_string {
        Some(redis_connection_string) => {
            println!("redis_connection_string: {}", redis_connection_string);
        }
        None => {}
    }
}
