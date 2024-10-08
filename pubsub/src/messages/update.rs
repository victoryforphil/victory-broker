use super::PubSubMessage;
use datastore::{
    datapoints::Datapoint,
    primitives::Primitives,
    time::VicInstantHandle,
    topics::{TopicKeyHandle, TopicKeyProvider},
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessage {
    pub topic: TopicKeyHandle,
    pub messages: Datapoint,
}

impl UpdateMessage {
    pub fn new(message: Datapoint) -> Self {
        UpdateMessage {
            topic: message.topic.clone(),
            messages: message,
        }
    }

    pub fn primitive<T: TopicKeyProvider>(
        topic: &T,
        time: VicInstantHandle,
        value: Primitives,
    ) -> Self {
        UpdateMessage {
            topic: topic.handle(),
            messages: Datapoint::new(topic, time, value),
        }
    }
}

impl From<UpdateMessage> for Datapoint {
    fn from(message: UpdateMessage) -> Self {
        message.messages
    }
}

impl From<Datapoint> for UpdateMessage {
    fn from(message: Datapoint) -> Self {
        UpdateMessage::new(message)
    }
}

impl From<UpdateMessage> for PubSubMessage {
    fn from(message: UpdateMessage) -> Self {
        PubSubMessage::Update(message)
    }
}

impl From<PubSubMessage> for UpdateMessage {
    fn from(message: PubSubMessage) -> Self {
        match message {
            PubSubMessage::Update(update) => update,
            _ => panic!("Invalid conversion from PubSubMessage to UpdateMessage"),
        }
    }
}
