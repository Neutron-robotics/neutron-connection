// #![deny(warnings)]
use super::connection_context::SharedConnectionContext;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde::Serialize;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::hyper::StatusCode;
use warp::ws::{Message, WebSocket};
use warp::Filter;

#[derive(Serialize)]
struct ClientInfo {
    clients_queue: Vec<String>,
    clients_active: Vec<String>,
}

pub async fn make_ws_context() {
    let shared_connection_context = SharedConnectionContext::default();
    let shared_connection_context = warp::any().map(move || shared_connection_context.clone());

    // GET /connection/:id -> websocket upgrade
    let connection = warp::path("connection")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::path::param::<String>())
        .and(warp::ws())
        // .and(users)
        .and(shared_connection_context.clone())
        .map(
            move |id: String, ws: warp::ws::Ws, shared_connection_context: SharedConnectionContext| {
                // This will call our function if the handshake succeeds.
                ws.on_upgrade(move |socket| user_connected(socket, shared_connection_context.clone(), id))
            },
        );

    // POST /register/:id
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::path::param::<String>())
        .and(shared_connection_context.clone())
        .and_then(register_client);

    let routes = connection.or(register);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn register_client(
    client_id: String,
    context: SharedConnectionContext,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Id is {client_id}");
    context.write().await.client_queue.insert(client_id);

    let response = ClientInfo {
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
            .clone()
            .keys()
            .map(|e| e.to_string())
            .collect::<Vec<String>>(),
    };

    let json_response = serde_json::to_string(&response).unwrap();

    Ok(warp::reply::with_status(json_response, StatusCode::OK))

    // Ok(warp::reply::with_status(
    //     format!(""),
    //     StatusCode::OK,
    // ))
}

async fn user_connected(ws: WebSocket, context: SharedConnectionContext, id: String) {
    if !context.read().await.client_queue.contains(&id) {
        eprintln!("Unautorized connection user, refusing {}", id);
        return;
    }

    eprintln!("new connection user: {}", id);

    context.write().await.client_queue.remove(&id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // Save the sender in our list of connected users.
    context.write().await.clients.insert(id.to_string(), tx);
    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = user_ws_rx.next().await {
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
    user_disconnected(&id, &context).await;
}

async fn user_message(my_id: &String, msg: Message, context: &SharedConnectionContext) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg);

    // New message from this user, send it to everyone else (except same uid)...
    for (uid, tx) in context.read().await.clients.iter() {
        if *my_id != *uid {
            if let Err(_disconnected) = tx.send(Message::text(new_msg.clone())) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}

async fn user_disconnected(my_id: &String, context: &SharedConnectionContext) {
    eprintln!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    context.write().await.clients.remove(my_id);
}