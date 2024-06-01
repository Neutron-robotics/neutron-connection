use super::command::Command;
use crate::network::{
    connection_context::SharedConnectionContext, model::base_message::BaseMessage,
};
use futures_util::SinkExt;
use log::{error, info};
use warp::filters::ws::Message;

pub async fn remove(command: Command, client_id: &String, context: &SharedConnectionContext) {
    if context.read().await.master_id != *client_id {
        error!(target: "connection_event", "{} is not master, aborting remove operation", client_id);
        return;
    }
    info!(
        target: "connection_event", "{} asked for removing sender: {:?}",
        client_id, &command.params
    );

    let sender: &mut futures_util::stream::SplitSink<warp::ws::WebSocket, warp::ws::Message>;
    let clients = &mut context.write().await.clients;
    if let Some(s) = clients.get_mut(&command.params) {
        sender = s;
    } else {
        error!(target: "connection_event", "Client with ID '{}' not found, cannot remove", &command.params);
        return;
    }

    let message = BaseMessage {
        message_type: "removedEvent".to_string(),
        message: {},
    };
    let json = serde_json::to_string(&message).unwrap();
    let message = Message::text(json.clone());

    if let Err(err) = sender.send(message).await {
        error!(target: "connection_event", "Failed to send remove command: {:?}", err);
    }

    if let Err(err) = sender.close().await {
        error!(target: "connection_event", "Failed to close sender: {:?}", err);
    }
}
