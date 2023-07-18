use futures_util::stream::SplitSink;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message as TMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use warp::ws::{Message, WebSocket};

pub struct ConnectionContext {
    pub clients: HashMap<String, SplitSink<WebSocket, Message>>,
    pub robot: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TMessage>>,
    pub master_id: String,
    pub client_queue: HashSet<String>,
    pub redis_connection: Option<redis::Connection>,
}

impl Default for ConnectionContext {
    fn default() -> ConnectionContext {
        ConnectionContext {
            clients: HashMap::new(),
            robot: None,
            master_id: "".to_string(),
            client_queue: HashSet::new(),
            redis_connection: None,
        }
    }
}

pub type SharedConnectionContext = Arc<RwLock<ConnectionContext>>;
