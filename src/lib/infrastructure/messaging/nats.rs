use crate::application::ports::messaging_ports::MessagingPort;
use async_nats::Client;
use futures::StreamExt;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;

#[derive(Clone)]
pub struct Nats {
    connection: Arc<Client>,
}

impl Nats {
    pub async fn new(addrs: &str) -> anyhow::Result<Nats> {
        let client = async_nats::connect(addrs).await?;

        Ok(Nats {
            connection: Arc::new(client),
        })
    }

    pub fn get_connection(&self) -> Arc<Client> {
        Arc::clone(&self.connection)
    }
}

impl MessagingPort for Nats {
    async fn publish_message(&self, topic: String, message: String) -> anyhow::Result<()> {
        let conn = self.get_connection();

        conn.publish(topic, message.into())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .map(|_| ())
    }
    async fn subscribe<F, T, Fut>(&self, topic: &str, handler: F) -> anyhow::Result<()>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
        T: DeserializeOwned + Send + Sync + Debug + 'static,
    {
        let conn = self.get_connection();

        let t = String::from(topic);

        let mut subscriber = conn
            .subscribe(t)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        tokio::spawn(async move {
            while let Some(message) = subscriber.next().await {
                let message_str = String::from_utf8_lossy(&message.payload).to_string();

                let parsed_message: T = match serde_json::from_str(&message_str) {
                    Ok(msg) => msg,
                    Err(e) => {
                        tracing::error!("Failed to parse message: {:?}", e);
                        continue;
                    }
                };

                if let Err(e) = handler(parsed_message).await {
                    tracing::error!("Failed to handle message: {:?}", e);
                }
            }
        });

        Ok(())
    }
}
