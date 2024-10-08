pub mod config;

use std::{collections::HashMap, sync::Arc};

use datastore::{
    database::Datastore,
    topics::{TopicKeyHandle, TopicKeyProvider},
};
use log::{debug, info};

use crate::{
    adapters::PubSubAdapterHandle,
    channel::{PubSubChannel, PubSubChannelHandle},
    client::{PubSubClient, PubSubClientHandle, PubSubClientIDType},
    messages::PubSubMessage,
    RwLockType,
};

pub struct PubSubServer {
    pub channels: HashMap<TopicKeyHandle, PubSubChannelHandle>,
    pub clients: HashMap<PubSubClientIDType, PubSubClientHandle>,
    pub adapters: Vec<PubSubAdapterHandle>,
    pub datastore: Datastore,
}

pub type PubSubServerHandle = RwLockType<PubSubServer>;

impl PubSubServer {
    pub fn new() -> Self {
        PubSubServer {
            channels: HashMap::new(),
            clients: HashMap::new(),
            adapters: Vec::new(),
            datastore: Datastore::new(),
        }
    }

    pub fn add_adapter(&mut self, adapter: PubSubAdapterHandle) {
        self.adapters.push(adapter);
    }

    pub fn handle() -> PubSubServerHandle {
        Arc::new(tokio::sync::RwLock::new(PubSubServer::new()))
    }
    pub fn tick(&mut self) {
        // 1. Read from adapters
        let mut incoming_msgs: HashMap<PubSubClientIDType, Vec<PubSubMessage>> = HashMap::new();
        for adapter in self.adapters.iter_mut() {
            let mut adapter = adapter.try_lock().unwrap();
            let msgs = adapter.read();
            for (client_id, messages) in msgs {
                incoming_msgs
                    .entry(client_id)
                    .or_insert_with(Vec::new)
                    .extend(messages);
            }
        }

        let mut to_send: HashMap<PubSubClientIDType, Vec<PubSubMessage>> = HashMap::new();
        for (client_id, messages) in incoming_msgs {
            for message in messages {
                let handle_to_send = self.handle_message(client_id, message);
                to_send
                    .entry(client_id)
                    .or_insert_with(Vec::new)
                    .extend(handle_to_send);
            }
        }

        for (_client_id, messages) in &to_send {
            for message in messages {
                let message = message.clone();
                match message {
                    PubSubMessage::Publish(message) => {
                        let topic = message.topic;
                        let channel = self.get_or_insert_channel(topic);
                        let mut channel = channel.try_lock().unwrap();
                        channel.on_publish(message.messages);
                    }
                    _ => {}
                }
            }
        }

        for channel in self.channels.values() {
            let mut channel = channel.try_lock().unwrap();
            let subscribers = channel.subscribers.clone();
            for subscriber in subscribers.iter() {
                let subscriber = subscriber.try_lock().unwrap();
                let updates = channel.get_updates(subscriber.id);
                for update in updates {
                    to_send
                        .entry(subscriber.id)
                        .or_insert_with(Vec::new)
                        .push(PubSubMessage::Update(update.clone()));
                }
            }
        }

        for adapter in self.adapters.iter_mut() {
            let mut adapter = adapter.try_lock().unwrap();
            adapter.write(to_send.clone());
        }
    }

    fn get_or_insert_channel(&mut self, topic: TopicKeyHandle) -> PubSubChannelHandle {
        let channel = self.channels.get(&topic);
        if channel.is_none() {
            debug!("Creating new channel for topic: {}", topic);
            let channel = PubSubChannel::try_new(topic.clone(), &mut self.datastore).unwrap();
            let channel_handle = channel.handle();
            self.channels.insert(topic, channel_handle.clone());
            return channel_handle;
        }
        channel.unwrap().clone()
    }

    fn register_client(&mut self, client_id: PubSubClientIDType) {
        let client = PubSubClient::new(client_id);
        let client_handle = Arc::new(tokio::sync::Mutex::new(client));
        self.clients.insert(client_id, client_handle);
        info!("Registered new client: {}", client_id);
    }

    fn subscribe_client(&mut self, client_id: PubSubClientIDType, topic: TopicKeyHandle) {
        info!("Client {} subscribing to topic: {}", client_id, topic);
        let channel = self.get_or_insert_channel(topic).clone();
        let client = self.clients.get(&client_id).unwrap();
        channel.try_lock().unwrap().add_subscriber(client.clone());
    }

    fn handle_message(
        &mut self,
        client_id: PubSubClientIDType,
        message: PubSubMessage,
    ) -> Vec<PubSubMessage> {
        let mut to_send = Vec::new();
        match message {
            PubSubMessage::Register() => {
                self.register_client(client_id);
                to_send.push(PubSubMessage::Welcome(client_id));
            }
            PubSubMessage::Publish(data) => {
                to_send.push(data.into());
            }
            PubSubMessage::Subscribe(data) => {
                self.subscribe_client(client_id, data.topic.handle());
            }
            PubSubMessage::Update(_) => {}
            PubSubMessage::Health() => {}
            PubSubMessage::Welcome(_) => {}
        }
        to_send
    }
}
