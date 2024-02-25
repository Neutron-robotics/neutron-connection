use crate::network::{
    connection_context::SharedConnectionContext, poll_robot_status::poll_robot_status,
};

pub async fn subscribe_robot_status(
    client_id: &String,
    context: &SharedConnectionContext,
) {
    if context.read().await.client_subscribed_robot_status.contains(client_id) {
        return;
    }

    println!("Subscribed client to robot status - {}", client_id);
    context
        .write()
        .await
        .client_subscribed_robot_status
        .push(client_id.clone());

    let connection_context_clone = context.clone();

    if context.write().await.client_subscribed_robot_status.len() == 1 {
        tokio::spawn(async move { poll_robot_status(&connection_context_clone).await });
    }
}
