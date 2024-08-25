use std::{collections::HashMap, sync::Arc};

use crate::{adapters::PubSubAdapter, client::PubSubClientIDType, messages::PubSubMessage};

pub struct TCPClientAdapter {}

impl TCPClientAdapter {
    pub fn new() -> TCPClientAdapter {
        TCPClientAdapter {}
    }
}

impl PubSubAdapter for TCPClientAdapter {
    fn read(&mut self) -> HashMap<PubSubClientIDType, Vec<PubSubMessage>> {
        todo!()
    }

    fn write(&mut self, to_send: HashMap<PubSubClientIDType, Vec<PubSubMessage>>) {
        todo!()
    }
}
