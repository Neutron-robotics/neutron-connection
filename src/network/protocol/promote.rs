use warp::filters::ws::Message;

use super::command::Command;
use crate::network::{
    connection_context::SharedConnectionContext, model::{base_message::BaseMessage, connection_infos::ClientInfo},
    ws_proxy::send_all_clients,
};

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

    let client_infos = ClientInfo::from_context(&*context.read().await);
    let base_message = BaseMessage {
        message_type: "connectionInfos".to_string(),
        message: client_infos,
    };
    let json = serde_json::to_string(&base_message).unwrap();
    let message = Message::text(json.clone());
    send_all_clients(&context, message).await;
}
