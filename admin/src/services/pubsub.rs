use crate::proto::pubsub_admin::{self, Adapter, ChannelResponse, PubSubChannel};
use std::time::Duration;

use log::info;

use pubsub::server::PubSubServerHandle;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status};
/// Provided by the requester and used by the manager task to send
/// the command response back to the requester.

pub struct PubSubAdmin {
    server: PubSubServerHandle,
}
impl PubSubAdmin {
    pub fn new(server: PubSubServerHandle) -> Self {
        PubSubAdmin { server }
    }
}
#[tonic::async_trait]
impl pubsub_admin::pub_sub_admin_service_server::PubSubAdminService for PubSubAdmin {
    type RequestChannelsStream = ReceiverStream<Result<ChannelResponse, Status>>;
    async fn request_channels(
        &self,
        request: Request<pubsub_admin::ChannelRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<Self::RequestChannelsStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(12);

        let server = self.server.clone();

        tokio::spawn(async move {
            loop {
                let channels = server.read().await.channels.clone();

                let channel_data = channels
                    .iter()
                    .map(|(topic, clients)| {
                        let clients = clients.try_lock().unwrap();
                        return PubSubChannel {
                            topic: topic.to_string(),
                            subscribers: clients
                                .subscribers
                                .iter()
                                .map(|client| client.try_lock().unwrap().id.to_string())
                                .collect(),
                            publishers: clients
                                .publishers
                                .iter()
                                .map(|client| client.try_lock().unwrap().id.to_string())
                                .collect(),
                            message_count: clients.get_queue_size() as i32,
                        };
                    })
                    .collect();

                let response = ChannelResponse {
                    channels: channel_data,
                };
                match tx.send(Ok(response)).await {
                    Ok(_) => {

                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(output_stream))
    }

    type RequestAdaptersStream = ReceiverStream<Result<pubsub_admin::AdapterResponse, Status>>;
    async fn request_adapters(
        &self,
        request: Request<pubsub_admin::AdapterRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<Self::RequestAdaptersStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(12);

        let server = self.server.clone();

        tokio::spawn(async move {
            loop {
                let adapters = server.read().await.adapters.clone();

                let adapter_data = adapters
                    .iter()
                    .map(|adapter| {
                        let adapter = adapter.try_lock().unwrap();
                        return Adapter {
                            name:adapter.get_name().to_string(),
                            live: adapter.get_live(),
                            description: adapter.get_description().to_string(),
                            stats:
                                adapter.get_stats().iter().map(|(key, value)| format!("{}: {}", key, value)).collect()
                        }

                    })
                    .collect();

                let response = pubsub_admin::AdapterResponse {
                    adapters: adapter_data,
                };
                match tx.send(Ok(response)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
                tokio::time::sleep(Duration::from_millis(250)).await;
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(output_stream))
    }
}
