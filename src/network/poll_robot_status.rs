use std::time::Duration;

use super::{
    connection_context::SharedConnectionContext, model::robot_status::RobotStatus,
    ws_proxy::send_client,
};
use crate::network::model::base_message::BaseMessage;
use tokio::time::{sleep, Instant};
use warp::filters::ws::Message;

pub async fn poll_robot_status(context: &SharedConnectionContext) {
    let client = reqwest::Client::new();
    let robot_url = format!(
        "http://{}:{}/robot/status",
        context.read().await.robot_hostname,
        8000 // todo - modify to robot port
    );

    loop {
        if context.read().await.client_subscribed_robot_status.len() == 0 {
            println!("Existing robot status loop as no handle is defined");
            return;
        }

        let start_time = Instant::now();
        let body = match client.get(&robot_url).send().await {
            Ok(value) => value,
            Err(error) => {
                println!("Error while fetching robot status: {}", error);
                sleep(Duration::from_millis(500)).await;
                continue;
            }
        };

        let text = match body.text().await {
            Ok(text) => text,
            Err(error) => {
                println!("Error while decoding robot status response: {}", error);
                sleep(Duration::from_millis(500)).await;
                continue;
            }
        };
        let elapsed_time = start_time.elapsed().as_millis();

        let mut robot_status: RobotStatus = match serde_json::from_str(&text) {
            Ok(value) => value,
            Err(error) => {
                println!("Error while decoding robot status response: {}", error);
                sleep(Duration::from_millis(500)).await;
                continue;
            }
        };

        robot_status.system.latency = Some(elapsed_time);

        println!("made obj");

        let base_message = BaseMessage {
            message_type: "robotStatus".to_string(),
            message: robot_status,
        };

        let serialized_message = serde_json::to_string(&base_message).unwrap();
        let message = Message::text(serialized_message);

        let client_ids = context.read().await.client_subscribed_robot_status.clone();
        for client_id in client_ids {
            println!("sending to client");
            send_client(context, &client_id, message.clone()).await;
        }

        sleep(Duration::from_millis(500)).await;
    }
}
