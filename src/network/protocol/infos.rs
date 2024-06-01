use crate::network::model::base_message::BaseMessage;
use crate::network::model::connection_infos::ClientInfo;
use crate::network::ws_proxy::{send_all_clients, send_other};
use crate::network::{connection_context::SharedConnectionContext, ws_proxy::send_client};
use warp::ws::Message;

pub async fn infos(client_id: &String, context: &SharedConnectionContext) {
    let message = build_info_message(context).await;
    send_client(context, client_id, message).await;
}

pub async fn send_info_all(context: &SharedConnectionContext) {
    let message = build_info_message(context).await;
    send_all_clients(context, message).await;
}

pub async fn send_info_others(client_id: &String, context: &SharedConnectionContext) {
    let message = build_info_message(context).await;
    send_other(context, client_id, message).await;
}

pub async fn build_info_message(context: &SharedConnectionContext) -> warp::ws::Message {
    let client_infos = ClientInfo::from_context(&*context.read().await);
    let base_message = BaseMessage {
        message_type: "connectionInfos".to_string(),
        message: client_infos,
    };
    let json = serde_json::to_string(&base_message).unwrap();
    return Message::text(json.clone());
}