use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use warp::ws::{Message, WebSocket};
use tokio_tungstenite::tungstenite::Message as TMessage;

pub struct ConnectionContext {
    pub clients: HashMap<String, SplitSink<WebSocket, Message>>,
    pub robot: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TMessage>>,
    pub master_id: String,
    pub client_queue: HashSet<String>,
}

impl Default for ConnectionContext {
    fn default() -> ConnectionContext {
        ConnectionContext {
            clients: HashMap::new(),
            robot: None,
            master_id: "".to_string(),
            client_queue: HashSet::new(),
        }
    }
}

pub type SharedConnectionContext = Arc<RwLock<ConnectionContext>>;
