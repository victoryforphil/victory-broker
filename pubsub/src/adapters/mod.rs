pub mod mock;
pub mod tcp;

use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{client::PubSubClientIDType, messages::PubSubMessage, MutexType};

pub trait PubSubAdapter {
    fn get_name(&self) -> String;
    fn read(&mut self) -> HashMap<PubSubClientIDType, Vec<PubSubMessage>>;
    fn write(&mut self, to_send: HashMap<PubSubClientIDType, Vec<PubSubMessage>>);
}
pub type PubSubAdapterHandle = MutexType<dyn PubSubAdapter + Send>;
