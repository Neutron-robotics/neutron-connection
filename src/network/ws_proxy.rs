use crate::network::model::connection_infos::ClientInfo;
use crate::network::ws_client::send_robot;

// #![deny(warnings)]
use super::connection_context::SharedConnectionContext;
use super::model::base_message::BaseMessage;
use super::protocol::command::{process_command, Command};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use warp::hyper::StatusCode;
use warp::ws::{Message, WebSocket};
use warp::{Filter, Reply};

pub async fn server_start(port: u16, shared_connection_context: SharedConnectionContext) {
    let shared_connection_context = warp::any().map(move || shared_connection_context.clone());

    // GET /connection/:id -> websocket upgrade
    let connection = warp::path("connection")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::path::param::<String>())
        .and(warp::ws())
        .and(shared_connection_context.clone())
        .map(
            move |id: String,
                  ws: warp::ws::Ws,
                  shared_connection_context: SharedConnectionContext| {
                // This will call our function if the handshake succeeds.
                ws.on_upgrade(move |socket| {
                    user_connected(socket, shared_connection_context.clone(), id)
                })
            },
        );

    // POST /register/:id
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::path::param::<String>())
        .and(shared_connection_context.clone())
        .and_then(register_client);

    let routes = connection.or(register);

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

async fn register_client(
    client_id: String,
    context: SharedConnectionContext,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Registering client {client_id}");
    if context.read().await.clients.contains_key(&client_id) {
        return Ok(
            warp::reply::with_status(warp::reply(), warp::http::StatusCode::BAD_REQUEST)
                .into_response(),
        );
    }

    context.write().await.client_queue.insert(client_id);
    let client_infos = ClientInfo::from_context(&*context.read().await);
    let json_response = serde_json::to_string(&client_infos).unwrap();
    Ok(warp::reply::with_status(json_response, StatusCode::OK).into_response())
}

async fn user_connected(ws: WebSocket, context: SharedConnectionContext, id: String) {
    if !context.read().await.client_queue.contains(&id) {
        eprintln!("Unautorized connection user, refusing {}", id);
        return;
    }
    context.write().await.client_queue.remove(&id);

    let (sender, mut receiver) = ws.split();

    // Save the sender in our list of connected users.
    context.write().await.client_connect(&id, sender);

    // motify connected clients about the new connection
    let client_infos = ClientInfo::from_context(&*context.read().await);
    let json = serde_json::to_string(&client_infos).unwrap();
    let message = Message::text(json.clone());
    send_other(&context, &id, message).await;

    // receive loop for current client
    while let Some(result) = receiver.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", id, e);
                break;
            }
        };
        user_message(&id, msg, &context).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    context.write().await.client_disconnect(&id);
    let client_infos = ClientInfo::from_context(&*context.read().await);
    let base_message = BaseMessage {
        message_type: "connectionInfos".to_string(),
        message: client_infos,
    };
    let json = serde_json::to_string(&base_message).unwrap();
    let message = Message::text(json.clone());
    send_other(&context, &id, message).await;
}

async fn user_message(my_id: &String, msg: Message, context: &SharedConnectionContext) {
    // Skip any non-Text messages...
    let msg_str = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg_str);
    println!("{}", new_msg);

    // Deserialize the JSON message into a serde_json::Value object
    let json: Value = match serde_json::from_str(&msg_str) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Failed to deserialize JSON message: {}", error);
            return;
        }
    };

    if json.get("op").is_some() {
        // Ros message, to be streamed to robot and forwarded to clients

        send_robot(context, &msg).await;
        send_other(context, my_id, msg).await;
    } else if json.get("command").is_some() {
        let command: Result<Command, _> = serde_json::from_value(json);
        if let Ok(command) = command {
            process_command(command, my_id, context).await;
        } else {
            eprintln!("Failed to deserialize Format2: {:?}", command);
        }
    } else {
        eprintln!("Unknown message {}", &msg_str);
    }
}

pub async fn send_other(context: &SharedConnectionContext, client_id: &String, message: Message) {
    // New message from this user, send it to everyone else (except same uid)...
    for (uid, sender) in context.write().await.clients.iter_mut() {
        if *client_id != *uid {
            if let Err(err) = sender.send(message.clone()).await {
                eprintln!("Fail to send other {:?}", err);
            }
        }
    }
}

pub async fn send_client(context: &SharedConnectionContext, client_id: &String, message: Message) {
    // Respond to the user defined by the client_id
    for (uid, sender) in context.write().await.clients.iter_mut() {
        if *client_id == *uid {
            if let Err(err) = sender.send(message.clone()).await {
                eprintln!("Fail to send client {:?}", err);
            }
        }
    }
}

pub async fn send_all_clients(context: &SharedConnectionContext, message: Message) {
    // Respond to the user defined by the client_id
    for client in context.write().await.clients.iter_mut() {
        if let Err(err) = client.1.send(message.clone()).await {
            eprintln!("Fail to send client {:?}", err);
        }
    }
}
