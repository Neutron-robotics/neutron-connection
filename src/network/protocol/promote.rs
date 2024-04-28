use log::error;
use warp::filters::ws::Message;

use super::{command::Command, infos::send_info_all};
use crate::network::{
    connection_context::SharedConnectionContext, model::{base_message::BaseMessage, connection_infos::ClientInfo},
    ws_proxy::send_all_clients,
};

pub async fn promote(command: Command, client_id: &String, context: &SharedConnectionContext) {
    if context.read().await.master_id != *client_id {
        error!(target: "connection_event", "{} is not master, promition forbidden", client_id);
        return;
    }

    if !context.read().await.clients.contains_key(&command.params) {
        error!(target: "connection_event", "{} not found, aborting promotion", command.params);
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
    send_info_all(context).await;
}
