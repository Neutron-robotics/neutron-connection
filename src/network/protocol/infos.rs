use warp::ws::Message;

use crate::network::{
    connection_context::SharedConnectionContext,
    ws_proxy::{send_client, ClientInfo},
};

pub async fn infos(client_id: &String, context: &SharedConnectionContext) {
    let client_infos = ClientInfo {
        clients_queue: context
            .read()
            .await
            .client_queue
            .clone()
            .into_iter()
            .collect::<Vec<String>>(),
        clients_active: context
            .read()
            .await
            .clients
            .keys()
            .map(|e| e.to_string())
            .collect::<Vec<String>>(),
    };
    let json = serde_json::to_string(&client_infos).unwrap();
    let message = Message::text(json.clone());
    send_client(context, client_id, message).await;
}
