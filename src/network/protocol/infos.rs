use crate::network::model::base_message::BaseMessage;
use crate::network::model::connection_infos::ClientInfo;
use crate::network::{connection_context::SharedConnectionContext, ws_proxy::send_client};
use warp::ws::Message;

pub async fn infos(client_id: &String, context: &SharedConnectionContext) {
    let client_infos = ClientInfo::from_context(&*context.read().await);
    let base_message = BaseMessage {
        message_type: "connectionInfos".to_string(),
        message: client_infos,
    };
    let json = serde_json::to_string(&base_message).unwrap();
    let message = Message::text(json.clone());
    send_client(context, client_id, message).await;
}
