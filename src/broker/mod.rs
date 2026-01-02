use async_trait::async_trait;
use futures_core::Stream;
use std::pin::Pin;

#[cfg(feature = "broker-nats")]
pub mod nats;

#[async_trait]
pub trait MessageBroker: Send + Sync {
    async fn publish(&self, topic: &str, payload: Vec<u8>) -> anyhow::Result<()>;

    async fn subscribe(
        &self,
        topic: &str,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = Vec<u8>> + Send>>>;
}
