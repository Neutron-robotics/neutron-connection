use super::command::Command;
use crate::network::connection_context::SharedConnectionContext;
use futures_util::SinkExt;

pub async fn remove(command: Command, client_id: &String, context: &SharedConnectionContext) {
    if context.read().await.master_id != *client_id {
        eprintln!("[Remove] {} is not master, access forbidden", client_id);
        return;
    }
    println!(
        "[Remove][{}] Closing sender: {:?}",
        client_id, &command.params
    );

    let sender: &mut futures_util::stream::SplitSink<warp::ws::WebSocket, warp::ws::Message>;
    let clients = &mut context.write().await.clients;
    if let Some(s) = clients.get_mut(&command.params) {
        sender = s;
    } else {
        eprintln!("Client with ID '{}' not found", &command.params);
        return;
    }

    if let Err(err) = sender.close().await {
        eprintln!("Failed to close sender: {:?}", err);
    }
}
