use futures_util::stream::{SplitStream, StreamExt};
use std::sync::{Arc, RwLock};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
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
    // while let Some(result) = receiver.next().await {
    //     let msg = match result {
    //         Ok(msg) => msg,
    //         Err(e) => {
    //             eprintln!("websocket error: {}", e);
    //             break;
    //         }
    //     };
    //     robot_message(msg, &connection_context).await;
    // }

    println!("Robot disconnected!");
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
    let msg_str = if let Ok(s) = msg.into_text() {
        s
    } else {
        return;
    };

    let new_msg = format!("<Robot>: {}", msg_str);
    println!("{}", new_msg);
}

// Spawn a task to handle sending messages asynchronously
// let send_handle: JoinHandle<Result<(), Box<dyn std::error::Error>>> = tokio::spawn(async move {
//     while let Some(message) = read_rx.recv().await {
//         if let Err(e) = ws_stream.send(message).await {
//             eprintln!("Failed to send message: {:?}", e);
//             break;
//         }
//     }

//     Ok(())
// });

// // Process received messages asynchronously
// while let Some(Ok(msg)) = ws_stream.next().await {
//     // Handle the received message
//     if let Some(robot) = connection_context.read().unwrap().robot.clone() {
//         // Do something with the robot object
//         // Set the sender as the robot
//         let _ = robot.send(msg.clone()).await;
//     } else {
//         eprintln!("Robot not available");
//     }
// }

// // Wait for the send task to complete
// let _ = send_handle.await?;

// Ok(())
// }

// pub struct WebSocketContext {
//     pub sender: Sender<OwnedMessage>,
//     pub handle: thread::JoinHandle<()>,
// }

// pub fn make_client(hostname: &str, port: &str) -> WebSocketContext {
//     let (sender, receiver) = channel();
//     let address = format!("ws://{}:{}", hostname, port);
//     let client = WebSocketClientBuilder::new(&address)
//         .unwrap()
//         .connect_insecure()
//         .unwrap();

//     let shared_client = Arc::new(Mutex::new(client));
//     let cloned_client = Arc::clone(&shared_client);

//     let handle = thread::spawn(move || {
//         loop {
//             match cloned_client.lock() {
//                 Ok(mut client) => {
//                     match client.recv_message() {
//                         Ok(message) => {
//                             if let OwnedMessage::Text(text) = message {
//                                 println!("Received message: {}", text);
//                             }
//                         }
//                         Err(err) => {
//                             eprintln!("Error receiving message: {:?}", err);
//                             break;
//                         }
//                     }
//                 }
//                 Err(err) => {
//                     eprintln!("Error acquiring lock: {:?}", err);
//                     break;
//                 }
//             }
//         }
//     });

//     WebSocketContext {
//         sender,
//         handle,
//     }
// }
