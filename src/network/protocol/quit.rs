use super::command::Command;
use crate::network::connection_context::SharedConnectionContext;
use futures_util::SinkExt;
use log::{error, info};

pub async fn quit(_: Command, client_id: &String, context: &SharedConnectionContext) {
    let sender: &mut futures_util::stream::SplitSink<warp::ws::WebSocket, warp::ws::Message>;
    let clients = &mut context.write().await.clients;
    if let Some(s) = clients.get_mut(client_id) {
        sender = s;
    } else {
        error!(target: "connection_event", "Client with ID '{}' not found, cannot quit", client_id);
        return;
    }

    if let Err(err) = sender.close().await {
        error!(target: "connection_event", "Failed to close sender: {:?}", err);
    }

    info!(target: "connection_event", "[{}] quit connection", client_id);
}
