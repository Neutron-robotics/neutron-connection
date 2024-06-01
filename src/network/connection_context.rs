use futures_util::stream::SplitSink;
use log::info;
use std::collections::{HashMap, HashSet};
use std::process::exit;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::tungstenite::Message as TMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use warp::ws::{Message, WebSocket};

pub struct ConnectionContext {
    pub id: String,
    pub clients: HashMap<String, SplitSink<WebSocket, Message>>,
    pub robot: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TMessage>>,
    pub master_id: String,
    pub client_queue: HashSet<String>,
    pub client_subscribed_robot_status: Vec<String>,
    pub redis_connection: Option<redis::Connection>,
    pub shutdown_handle: Option<JoinHandle<()>>,
    pub robot_status_pull_handle: Option<JoinHandle<()>>,
    pub application_timeout: Option<u64>,
    pub robot_hostname: String,
    pub robot_port: u16
}

impl Default for ConnectionContext {
    fn default() -> ConnectionContext {
        ConnectionContext {
            id: "".to_string(),
            clients: HashMap::new(),
            robot: None,
            master_id: "".to_string(),
            client_queue: HashSet::new(),
            client_subscribed_robot_status: Vec::new(),
            redis_connection: None,
            shutdown_handle: None,
            robot_status_pull_handle: None,
            application_timeout: None,
            robot_hostname: "".to_string(),
            robot_port: 0
        }
    }
}

impl ConnectionContext {
    pub fn client_connect(&mut self, client_id: &String, sender: SplitSink<WebSocket, Message>) {
        info!(target: "connection_event", "Client connected: {}", client_id);

        if self.master_id.is_empty() && self.clients.is_empty() {
            self.master_id = client_id.to_string();
        }

        self.clients.insert(client_id.to_string(), sender);

        if let Some(timer) = self.shutdown_handle.take() {
            timer.abort();
        }
    }

    pub fn client_disconnect(&mut self, client_id: &String) {
        info!(target: "connection_event", "Client disconnected : {}", client_id);

        self.clients.remove(client_id);
        self.client_subscribed_robot_status
            .retain(|s| s != client_id);

        if self.clients.len() == 0 && self.application_timeout.is_some() {
            self.start_shutdown_timer(self.application_timeout.unwrap());
        }

        if self.client_subscribed_robot_status.len() == 0 {
            if let Some(handle) = self.robot_status_pull_handle.take() {
                info!(target: "connection_event", "Stopping robot polling (NO_CLIENT_CONNECTED");
                handle.abort();
            }
        }
    }

    fn start_shutdown_timer(&mut self, timeout: u64) {
        let join_handle = tokio::spawn(async move {
            info!(target: "init", "No connected client, initiating shutdown procedure ({timeout} seconds)");
            sleep(Duration::from_secs(timeout)).await;
            info!(target: "init", "Shutting down the app...");
            exit(0);
        });

        self.shutdown_handle = Some(join_handle);
    }
}

pub type SharedConnectionContext = Arc<RwLock<ConnectionContext>>;
