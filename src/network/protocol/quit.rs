use super::command::Command;
use crate::network::connection_context::SharedConnectionContext;
use futures_util::SinkExt;

pub async fn quit(_: Command, client_id: &String, context: &SharedConnectionContext) {
    let sender: &mut futures_util::stream::SplitSink<warp::ws::WebSocket, warp::ws::Message>;
    let clients = &mut context.write().await.clients;
    if let Some(s) = clients.get_mut(client_id) {
        sender = s;
    } else {
        eprintln!("Client with ID '{}' not found", client_id);
        return;
    }

    if let Err(err) = sender.close().await {
        eprintln!("Failed to close sender: {:?}", err);
    }

    println!("[QUIT][{}] Quiting", client_id);
}
