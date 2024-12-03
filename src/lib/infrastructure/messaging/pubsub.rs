use crate::application::ports::messaging_ports::MessagingPort;
use anyhow::Result;
use futures::StreamExt;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::SubscriptionConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct PubSubMessaging {
    client: Arc<Client>,
    project_id: String,
}

impl PubSubMessaging {
    pub async fn new(project_id: String) -> Result<Self> {
        let config = ClientConfig::default().with_auth().await?;
        let client = Client::new(config).await?;

        Ok(PubSubMessaging {
            client: Arc::new(client),
            project_id,
        })
    }
}

impl MessagingPort for PubSubMessaging {
    async fn publish_message(&self, topic: String, message: String) -> Result<()> {
        let t = format!("projects/{}/topics/{}", self.project_id, topic);
        let topic = self.client.topic(&t);

        if !topic.exists(None).await? {
            tracing::error!("Topic does not exist");
        }

        let publisher = topic.new_publisher(None);

        let msg = PubsubMessage {
            data: message.into(),
            ordering_key: "order".into(),
            ..Default::default()
        };

        let awaiter = publisher.publish(msg).await;

        awaiter.get().await?;

        Ok(())
    }

    async fn subscribe<F, T, Fut>(&self, queue: &str, handler: F) -> Result<()>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
        T: serde::de::DeserializeOwned + Send + Sync + std::fmt::Debug + 'static,
    {
        let _topic = self.client.topic(queue);

        let _config = SubscriptionConfig {
            enable_message_ordering: true,
            ..Default::default()
        };

        let t = format!("projects/nathael-dev/topics/{}", queue);

        let subscription = self.client.subscription(&t);
        let mut stream = subscription.subscribe(None).await?;

        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                let msg: Vec<u8> = message.message.data.clone();
                let msg = match String::from_utf8(msg) {
                    Ok(s) => s,
                    Err(e) => {
                        tracing::error!("Failed to parse message payload: {:?}", e);
                        continue;
                    }
                };

                let parsed_message: T = match serde_json::from_str(&msg) {
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
