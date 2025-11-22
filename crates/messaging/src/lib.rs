use std::sync::Arc;

use bytes::Bytes;
use chrono::{DateTime, Utc};
use common_config::IggyConfig;
use iggy::prelude::{
    IggyClient, IggyDuration, IggyError, IggyMessage, IggyProducer, IggyProducerConfig,
    IggyStreamProducer,
};
use serde::Serialize;
use std::time::Duration;
use thiserror::Error;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct IggyPublisher {
    _client: Arc<IggyClient>,
    producer: Arc<IggyProducer>,
}

impl IggyPublisher {
    pub async fn connect(config: &IggyConfig) -> Result<Self, MessagingError> {
        let linger = IggyDuration::from(Duration::from_millis(config.linger_ms));
        let producer_config = IggyProducerConfig::from_stream_topic(
            &config.stream,
            &config.topic,
            config.batch_size,
            linger,
        )?;

        let (client, producer) =
            IggyStreamProducer::with_client_from_url(&config.connection_string, &producer_config)
                .await?;
        producer.init().await?;
        info!(stream = %config.stream, topic = %config.topic, "connected to Iggy");
        Ok(Self {
            _client: Arc::new(client),
            producer: Arc::new(producer),
        })
    }

    pub async fn publish_json<T>(&self, payload: &T) -> Result<(), MessagingError>
    where
        T: Serialize,
    {
        let bytes = serde_json::to_vec(payload)?;
        self.publish_bytes(Bytes::from(bytes)).await
    }

    pub async fn publish_bytes(&self, payload: Bytes) -> Result<(), MessagingError> {
        let message = IggyMessage::builder().payload(payload).build()?;
        self.producer.send_one(message).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct EventEnvelope<T> {
    pub id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    pub fn new(payload: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            occurred_at: Utc::now(),
            payload,
        }
    }
}

#[derive(Debug, Error)]
pub enum MessagingError {
    #[error(transparent)]
    Iggy(#[from] IggyError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
