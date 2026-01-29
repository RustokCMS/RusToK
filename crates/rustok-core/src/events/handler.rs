use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use super::bus::EventBus;
use super::types::{DomainEvent, EventEnvelope};

#[async_trait]
pub trait EventHandler: Send + Sync {
    fn handles(&self, _event: &DomainEvent) -> bool {
        true
    }

    async fn handle(&self, envelope: &EventEnvelope) -> Result<()>;
}

pub struct EventDispatcher {
    bus: EventBus,
    handlers: Vec<Arc<dyn EventHandler>>,
}

impl EventDispatcher {
    pub fn new(bus: EventBus) -> Self {
        Self {
            bus,
            handlers: Vec::new(),
        }
    }

    pub fn register<H: EventHandler + 'static>(&mut self, handler: H) {
        self.handlers.push(Arc::new(handler));
    }

    pub fn start(self) {
        tokio::spawn(async move {
            let mut receiver = self.bus.subscribe();
            tracing::info!("EventDispatcher started");

            while let Ok(envelope) = receiver.recv().await {
                for handler in &self.handlers {
                    if handler.handles(&envelope.payload) {
                        let handler = Arc::clone(handler);
                        let envelope = envelope.clone();

                        tokio::spawn(async move {
                            if let Err(error) = handler.handle(&envelope).await {
                                tracing::error!(
                                    event_id = ?envelope.id,
                                    ?error,
                                    "Event handler failed"
                                );
                            }
                        });
                    }
                }
            }
        });
    }
}
