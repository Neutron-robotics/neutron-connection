use serde::Serialize;

use crate::network::connection_context::ConnectionContext;

#[derive(Serialize)]
pub struct ClientInfo {
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "clients")]
    pub clients_active: Vec<String>,
    #[serde(rename = "clientsQueue")]
    pub clients_queue: Vec<String>,
    #[serde(rename = "leaderId")]
    pub leader_id: String,
}

impl ClientInfo {
    pub fn from_context(context: &ConnectionContext) -> ClientInfo {
        ClientInfo {
            connection_id: context.id.clone(),
            clients_queue: context
                .client_queue
                .clone()
                .into_iter()
                .collect::<Vec<String>>(),
            clients_active: context
                .clients
                .keys()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            leader_id: context.master_id.clone(),
        }
    }
}
