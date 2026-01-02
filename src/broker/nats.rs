use super::MessageBroker;
use async_trait::async_trait;
use futures_util::StreamExt;
use std::pin::Pin;

#[derive(Clone)]
pub struct NatsBroker {
    client: async_nats::Client,
}

impl NatsBroker {
    pub async fn connect(url: &str) -> Result<Self, async_nats::ConnectError> {
        let client = async_nats::connect(url).await?;
        Ok(Self { client })
    }
}

#[async_trait]
impl MessageBroker for NatsBroker {
    async fn publish(&self, topic: &str, payload: Vec<u8>) -> anyhow::Result<()> {
        self.client
            .publish(topic.to_string(), payload.into())
            .await
            .map_err(|e| anyhow::anyhow!("NATS publish error: {}", e))?;
        Ok(())
    }

    async fn subscribe(
        &self,
        topic: &str,
    ) -> anyhow::Result<Pin<Box<dyn futures_core::Stream<Item = Vec<u8>> + Send>>> {
        let subscriber = self
            .client
            .subscribe(topic.to_string())
            .await
            .map_err(|e| anyhow::anyhow!("NATS subscribe error: {}", e))?;

        // Map NATS Message to generic Vec<u8>
        let stream = subscriber.map(|msg| msg.payload.to_vec());

        Ok(Box::pin(stream))
    }
}
