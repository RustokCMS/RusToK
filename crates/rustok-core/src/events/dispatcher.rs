use std::sync::Arc;

use tokio::task::JoinHandle;

use super::{EventBus, EventEnvelope, EventHandler};

#[derive(Clone, Debug)]
pub struct DispatcherConfig {
    pub channel_capacity: usize,
}

impl Default for DispatcherConfig {
    fn default() -> Self {
        Self {
            channel_capacity: 128,
        }
    }
}

pub type HandlerResult = crate::Result<()>;

#[derive(Debug)]
pub struct EventDispatcher {
    bus: EventBus,
    handlers: Vec<Arc<dyn EventHandler>>,
    config: DispatcherConfig,
}

impl EventDispatcher {
    pub fn new(bus: EventBus) -> Self {
        Self {
            bus,
            handlers: Vec::new(),
            config: DispatcherConfig::default(),
        }
    }

    pub fn with_config(mut self, config: DispatcherConfig) -> Self {
        self.config = config;
        self
    }

    pub fn register(&mut self, handler: Arc<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub fn start(self) -> RunningDispatcher {
        let mut receiver = self.bus.subscribe();
        let handlers = self.handlers;
        let _capacity = self.config.channel_capacity;

        let handle = tokio::spawn(async move {
            while let Ok(envelope) = receiver.recv().await {
                dispatch_event(&envelope, &handlers);
            }
        });

        RunningDispatcher { handle }
    }
}

#[derive(Debug)]
pub struct HandlerBuilder {
    bus: EventBus,
    handlers: Vec<Arc<dyn EventHandler>>,
    config: DispatcherConfig,
}

impl HandlerBuilder {
    pub fn new(bus: EventBus) -> Self {
        Self {
            bus,
            handlers: Vec::new(),
            config: DispatcherConfig::default(),
        }
    }

    pub fn config(mut self, config: DispatcherConfig) -> Self {
        self.config = config;
        self
    }

    pub fn handler(mut self, handler: Arc<dyn EventHandler>) -> Self {
        self.handlers.push(handler);
        self
    }

    pub fn build(self) -> EventDispatcher {
        EventDispatcher {
            bus: self.bus,
            handlers: self.handlers,
            config: self.config,
        }
    }
}

#[derive(Debug)]
pub struct RunningDispatcher {
    handle: JoinHandle<()>,
}

impl RunningDispatcher {
    pub fn abort(self) {
        self.handle.abort();
    }
}

fn dispatch_event(envelope: &EventEnvelope, handlers: &[Arc<dyn EventHandler>]) {
    for handler in handlers {
        if handler.handles(&envelope.event) {
            if let Err(error) = handler.handle(envelope) {
                tracing::error!(
                    handler = handler.name(),
                    error = ?error,
                    "Event handler failed"
                );
            }
        }
    }
}
