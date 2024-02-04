use serde::Serialize;

#[derive(Serialize)]
pub struct BaseMessage<T> {
    #[serde(rename = "messageType")]
    pub message_type: String,
    pub message: T,
}
