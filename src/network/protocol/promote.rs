use super::command::Command;
use crate::network::connection_context::SharedConnectionContext;

pub async fn promote(command: Command, client_id: &String, context: &SharedConnectionContext) {
    if context.read().await.master_id != *client_id {
        eprintln!("[Promote] {} is not master, access forbidden", client_id);
        return;
    }

    if !context.read().await.clients.contains_key(&command.params) {
        eprintln!("[Promote] {} not found", command.params);
        return;
    }

    context.write().await.master_id = command.params;
}
