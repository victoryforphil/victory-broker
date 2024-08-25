use std::{
    collections::{BTreeMap, HashMap}, net::TcpListener, sync::Arc
};

use tokio::{net::TcpSocket, sync::Mutex};

use crate::{adapters::PubSubAdapter, client::PubSubClientIDType, messages::PubSubMessage};


struct ListenerAgent{
    listener: TcpListener,
    clients: BTreeMap<PubSubClientIDType, TcpSocket>,
    tcp_out: tokio::sync::mpsc::Sender<(PubSubClientIDType, PubSubMessage)>,
    tcp_in: tokio::sync::mpsc::Receiver<(PubSubClientIDType, PubSubMessage)>,
}

impl ListenerAgent {
    async fn start_listener(&mut self) {
        loop {
            let (socket, _) = self.listener.accept().await?;
            let client_id = self.clients.len() as PubSubClientIDType;
            self.clients.insert(client_id, socket);
        }
    }
}

type ListenerAgentHandle = Arc<Mutex<ListenerAgent>>

pub struct TCPServerAdapter {
    agent: ListenerAgentHandle,
}

impl TCPServerAdapter {
    pub fn new() -> TCPServerAdapter {
        let (tcp_out, tcp_in) = tokio::sync::mpsc::channel(100);
        TCPServerAdapter {
            agent: Arc::new(Mutex::new(ListenerAgent {
                listener: TcpListener::bind("
        }
    }

    pub fn start_server(&mut self) {
        tokio::spawn(async move {
            self.start_listener().await;
        });
    }

    async fn start_listener(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:7001").await?;

    }
    async fn new_client(&mut self) {
        todo!()
    }
}

impl PubSubAdapter for TCPServerAdapter {
    fn read(&mut self) -> HashMap<PubSubClientIDType, Vec<PubSubMessage>> {
        let results =
    }

    fn write(&mut self, to_send: HashMap<PubSubClientIDType, Vec<PubSubMessage>>) {
        todo!()
    }
}
