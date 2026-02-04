pub mod auth;
pub mod cache;
pub mod context;
pub mod error;
pub mod events;
pub mod id;
pub mod migrations;
pub mod module;
pub mod permissions;
pub mod rbac;
pub mod registry;
pub mod types;
pub use cache::{CacheStats, InMemoryCacheBackend};
pub use context::{AppContext, CacheBackend, SearchBackend};
pub use error::{Error, Result};
pub use events::{
    DispatcherConfig, DomainEvent, EventBus, EventBusStats, EventDispatcher, EventEnvelope,
    EventHandler, EventTransport, HandlerBuilder, HandlerResult, MemoryTransport, ReliabilityLevel,
    RunningDispatcher,
};
pub use id::generate_id;
pub use migrations::ModuleMigration;
pub use module::{EventListener, MigrationSource};
pub use module::{HealthStatus, ModuleContext, RusToKModule};
pub use permissions::{Action, Permission, Resource};
pub use rbac::{PermissionScope, Rbac, SecurityContext};
pub use registry::ModuleRegistry;
pub use types::{UserRole, UserStatus};

pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::events::{
        DispatcherConfig, DomainEvent, EventBus, EventBusStats, EventDispatcher, EventEnvelope,
        EventHandler, EventTransport, HandlerBuilder, HandlerResult, MemoryTransport,
        ReliabilityLevel, RunningDispatcher,
    };
    pub use crate::id::generate_id;
    pub use crate::module::HealthStatus;
    pub use crate::permissions::{Action, Permission, Resource};
    pub use crate::rbac::{PermissionScope, Rbac, SecurityContext};
    pub use crate::types::{UserRole, UserStatus};
    pub use crate::{AppContext, CacheBackend, CacheStats, InMemoryCacheBackend, SearchBackend};
    pub use uuid::Uuid;
}
