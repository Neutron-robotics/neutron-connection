use serde::{Deserialize, Serialize};

use crate::network::connection_context::SharedConnectionContext;

use crate::network::protocol::infos::infos;
use crate::network::protocol::promote::promote;
use crate::network::protocol::quit::quit;
use crate::network::protocol::remove::remove;

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub command: String,
    pub params: String,
}

pub async fn process_command(
    command: Command,
    client_id: &String,
    context: &SharedConnectionContext,
) {
    match command.command.as_str() {
        "promote" => promote(command, client_id, context).await,
        "remove" => remove(command, client_id, context).await,
        "infos" => infos(client_id, context).await,
        "quit" => quit(command, client_id, context).await,
        _ => println!("Command not supported {}", command.command.as_str()),
    }
}
