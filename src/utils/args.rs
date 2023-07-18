use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about="The official Neutron websocket connection server that is in charge of transporting and managing network interactions for individual robot connections", long_about = None)]
pub struct Args {
    #[clap(long)]
    /// The id of the connection
    pub id: String,

    #[clap(long, short, short = 'm')]
    /// The master id for the connection
    pub connection_master: String,

    #[clap(long, short, short = 'r')]
    /// The robot id for the connection
    pub robot_id: String,

    #[clap(long, short, short = 'c')]
    /// The robot id for the connection
    pub redis_connection_string: String,
}

pub fn print_args(args: Args) {
    println!("Neutron Connection started!");
    println!("Args:");
    println!("id: {}", args.id);
    println!("connection_master: {}", args.connection_master);
    println!("robot_id: {}", args.robot_id);
    println!("redis_connection_string: {}", args.redis_connection_string);
}
