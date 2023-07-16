use super::connection_context::SharedConnectionContext;
use futures_util::stream::{SplitStream, StreamExt};
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub async fn websocket_client(
    connection_context: &SharedConnectionContext,
    hostname: &str,
    port: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("ws://{}:{}/", hostname, port);

    // Establish the WebSocket connection
    let (ws_stream, _) = connect_async(url).await?;

    let (sender, mut receiver) = ws_stream.split();

    let connection_context_clone = connection_context.clone();
    connection_context.write().await.robot = Some(sender);
    tokio::spawn(async move { process_socket(&connection_context_clone, &mut receiver).await });

    println!("Robot connected!");
    Ok(())
}

async fn process_socket(
    context: &SharedConnectionContext,
    receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    while let Some(result) = receiver.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        };
        robot_message(msg, context).await;
    }
}

async fn robot_message(msg: Message, context: &SharedConnectionContext) {
    let msg_str = if let Ok(s) = msg.clone().into_text() {
        s
    } else {
        return;
    };

    let new_msg = format!("<Robot>: {}", msg_str);
    println!("{}", new_msg);

    forward_clients(context, msg).await;
}

pub async fn send_robot(context: &SharedConnectionContext, message: &warp::filters::ws::Message) {
    let payload: &[u8] = message.as_bytes(); // Extract payload as &[u8]
    let tungstenite_message: Message = Message::binary(payload.to_owned());

    if let Some(robot) = &mut context.write().await.robot {
        match robot.send(tungstenite_message).await {
            Ok(()) => {}
            Err(err) => {
                print!("Error while sending message to robot {}", err.to_string());
            }
        }
    }
}

pub async fn forward_clients(context: &SharedConnectionContext, msg: Message) {
    let payload = msg.into_data(); // Extract payload as &[u8]
    let warp_message: warp::filters::ws::Message =
        warp::filters::ws::Message::binary(payload.to_owned());

    for (_, sender) in context.write().await.clients.iter_mut() {
        if let Err(err) = sender.send(warp_message.clone()).await {
            eprintln!("Fail to send other {:?}", err);
        }
    }
}
