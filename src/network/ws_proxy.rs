use crate::network::model::connection_infos::ClientInfo;
use crate::network::ws_client::send_robot;

// #![deny(warnings)]
use super::connection_context::SharedConnectionContext;
use super::protocol::command::{process_command, Command};
use super::protocol::infos::send_info_others;
use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde_json::Value;
use std::str;
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

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

async fn register_client(
    client_id: String,
    context: SharedConnectionContext,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!(target: "connection_event", "Registering client {client_id}");
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
        warn!(target: "connection_event", "Unautorized connection user, refusing {}", id);
        return;
    }
    context.write().await.client_queue.remove(&id);

    let (sender, mut receiver) = ws.split();

    // Save the sender in our list of connected users.
    context.write().await.client_connect(&id, sender);

    // motify connected clients about the new connection
    send_info_others(&id, &context).await;

    // receive loop for current client
    info!(target: "connection_msg", "TMP waiting for msgs!");
    while let Some(result) = receiver.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!(target: "connection_event", "websocket error(uid={}): {}", id, e);
                break;
            }
        };
        user_message(&id, msg, &context).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    context.write().await.client_disconnect(&id);

    // inform other users
    send_info_others(&id, &context).await;
}

async fn user_message(my_id: &String, msg: Message, context: &SharedConnectionContext) {
    // Skip any non-Text messages...

    info!(target: "connection_msg", "TMP received raw message: {:?}", msg);

    // let msg_bytes = msg.as_bytes();
    // let msg_str = str::from_utf8(msg_bytes).unwrap_or_else(|_| "<invalid UTF-8>");

    // info!(target: "connection_msg", "TMP try to deserialize: {}", msg_str);

    // let msg_str = if let Ok(s) = msg.to_str() {
    //     s
    // } else {
    //     return;
    // };

    // info!(target: "connection_msg", "TMP received message !");

    // d

    let msg_str = if let Ok(text) = msg.to_str() {
        info!(target: "connection_msg", "TMP received text message: {}", text);
        text.to_string()
    } else {
        // If it's not a text message, handle it as bytes and attempt to convert to UTF-8
        let msg_bytes = msg.as_bytes();
        match str::from_utf8(msg_bytes) {
            Ok(text) => {
                info!(target: "connection_msg", "TMP received binary message, converted to text: {}", text);
                text.to_string()
            }
            Err(_) => {
                error!(target: "connection_msg", "Received non-UTF-8 binary WebSocket message");
                return;
            }
        }
    };

    // e

    // Deserialize the JSON message into a serde_json::Value object
    let json: Value = match serde_json::from_str(&msg_str) {
        Ok(value) => value,
        Err(error) => {
            error!(target: "connection_event", "Failed to deserialize JSON message: {}", error);
            return;
        }
    };

    let op = json.get("op");
    if let Some(op_value) = op {
        // Ros message, to be streamed to robot and forwarded to clients
        info!(target: "connection_msg", "[user#{my_id}] [ROS] [{}]", op_value.to_string().trim_matches('"'));
        send_robot(context, &msg).await;
        send_other(context, my_id, msg).await;
    } else if json.get("command").is_some() {
        let command: Result<Command, _> = serde_json::from_value(json);
        if let Ok(command) = command {
            info!(target: "connection_msg", "[user#{my_id}] [COMMAND] [{}]", command.command);
            process_command(command, my_id, context).await;
        } else {
            error!(target: "connection_event","Failed to deserialize Format: {:?}", command);
        }
    } else {
        warn!(target: "connection_event","Unknown message received from user {}", &msg_str);
    }
}

pub async fn send_other(context: &SharedConnectionContext, client_id: &String, message: Message) {
    // New message from this user, send it to everyone else (except same uid)...
    for (uid, sender) in context.write().await.clients.iter_mut() {
        if *client_id != *uid {
            if let Err(err) = sender.send(message.clone()).await {
                error!(target: "connection_event", "Fail to send other {:?}", err);
            }
        }
    }
}

pub async fn send_client(context: &SharedConnectionContext, client_id: &String, message: Message) {
    // Respond to the user defined by the client_id
    for (uid, sender) in context.write().await.clients.iter_mut() {
        if *client_id == *uid {
            if let Err(err) = sender.send(message.clone()).await {
                error!(target: "connection_event", "Fail to send client {:?}", err);
            }
        }
    }
}

pub async fn send_all_clients(context: &SharedConnectionContext, message: Message) {
    // Respond to the user defined by the client_id
    for client in context.write().await.clients.iter_mut() {
        if let Err(err) = client.1.send(message.clone()).await {
            error!(target: "connection_event", "Fail to send client {:?}", err);
        }
    }
}
