use futures_util::stream::SplitSink;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};

pub struct ConnectionContext {
    pub clients: HashMap<String, SplitSink<WebSocket, Message>>,
    pub robot: mpsc::UnboundedSender<Message>,
    pub master_id: String,
    pub client_queue: HashSet<String>,
}

impl Default for ConnectionContext {
    fn default() -> ConnectionContext {
        ConnectionContext {
            clients: HashMap::new(),
            robot: mpsc::unbounded_channel().0,
            master_id: "".to_string(),
            client_queue: HashSet::new(),
        }
    }
}

pub type SharedConnectionContext = Arc<RwLock<ConnectionContext>>;
