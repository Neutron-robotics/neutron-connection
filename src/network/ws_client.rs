use futures_util::stream::{SplitStream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use super::connection_context::SharedConnectionContext;

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

async fn robot_message(msg: Message, _: &SharedConnectionContext) {
    let msg_str = if let Ok(s) = msg.into_text() {
        s
    } else {
        return;
    };

    let new_msg = format!("<Robot>: {}", msg_str);
    println!("{}", new_msg);
}