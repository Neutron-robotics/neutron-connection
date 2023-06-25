use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::ws::Message;

pub struct ConnectionContext {
    pub clients: HashMap<String, mpsc::UnboundedSender<Message>>,
    pub robot: mpsc::UnboundedSender<Message>,
    pub master_id: usize,
    pub client_queue: HashSet<String>,
}

impl Default for ConnectionContext {
    fn default() -> ConnectionContext {
        ConnectionContext {
            clients: HashMap::new(),
            robot: mpsc::unbounded_channel().0,
            master_id: 0,
            client_queue: HashSet::new(),
        }
    }
}

pub type SharedConnectionContext = Arc<RwLock<ConnectionContext>>;
