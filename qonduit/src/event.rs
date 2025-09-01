use crate::registry::EventHandlerRegistry;
use async_trait::async_trait;
use std::any::Any;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

/// A domain/event-sourcing style notification that has occurred in the system.
///
/// Events are immutable facts - they represent something that already happened.
/// They are broadcast to all registered handlers. Multiple handlers may react
/// to the same event type (fan-out).
///
/// Implementors should usually be simple data structures (often `struct` with
/// `#[derive(Clone, Debug)]`). The [`Clone`] bound enables the bus to deliver
/// the same event instance to multiple handlers.
///
/// # Example
/// ```
/// use qonduit::event::Event;
///
/// #[derive(Clone, Debug)]
/// struct ProductCreatedEvent {
///     pub id: u64,
///     pub name: String,
/// }
///
/// impl Event for ProductCreatedEvent {}
/// ```
pub trait Event: Clone + Send + Sync + Any + Debug {}

/// A handler that reacts to an event of type `E`.
///
/// Event handlers perform side effects or trigger follow‑up processes in reaction
/// to events (e.g. sending emails, updating projections, cache invalidation).
///
/// Handlers must be idempotent whenever possible because the same event might
/// be re-dispatched (e.g. in retry scenarios in higher-level architectures).
///
/// # Example
/// ```
/// use qonduit::async_trait;
/// use qonduit::event::{Event, EventHandler};
///
/// #[derive(Clone, Debug)]
/// struct InventoryLowEvent {
///     pub sku: String,
///     pub remaining: u32,
/// }
/// impl Event for InventoryLowEvent {}
///
/// struct NotifyTeamHandler;
///
/// #[async_trait]
/// impl EventHandler<InventoryLowEvent> for NotifyTeamHandler {
///     async fn handle(&self, event: InventoryLowEvent)
///         -> Result<(), Box<dyn std::error::Error + Send + Sync>>
///     {
///         # let _ = event;
///         // Send a notification to the operations team.
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait EventHandler<E: Event>: Send + Sync {
    /// Handles (reacts to) an event.
    ///
    /// Returning `Ok(())` signals successful processing. Returning an error will
    /// cause [`EventBus::dispatch`] to propagate the failure (and stop dispatching
    /// remaining handlers).
    async fn handle(&self, event: E) -> Result<(), Box<dyn Error + Send + Sync>>;
}

/// A lightweight publish/subscribe dispatcher for events.
///
/// The `EventBus` retrieves all handlers registered for the event's concrete
/// type and invokes each handler sequentially. Each handler receives a cloned
/// instance of the event value. If any handler returns an error, dispatching
/// stops and the error is returned to the caller.
///
/// Handlers are stored in an [`EventHandlerRegistry`]. You can construct a bus
/// manually or via the `event_bus!` macro.
///
/// # Example
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// use qonduit::async_trait;
/// use qonduit::event::{Event, EventHandler, EventBus};
/// use qonduit::registry::EventHandlerRegistry;
///
/// #[derive(Clone, Debug)]
/// struct ProductCreatedEvent { id: u64, name: String }
/// impl Event for ProductCreatedEvent {}
///
/// struct LogHandler;
/// struct ProjectionHandler;
///
/// #[async_trait]
/// impl EventHandler<ProductCreatedEvent> for LogHandler {
///     async fn handle(&self, e: ProductCreatedEvent)
///         -> Result<(), Box<dyn std::error::Error + Send + Sync>>
///     {
///         println!("LOG: product created {}", e.id);
///         Ok(())
///     }
/// }
///
/// #[async_trait]
/// impl EventHandler<ProductCreatedEvent> for ProjectionHandler {
///     async fn handle(&self, e: ProductCreatedEvent)
///         -> Result<(), Box<dyn std::error::Error + Send + Sync>>
///     {
///         // Update read model / projection
///         println!("PROJECTION update for {}", e.id);
///         Ok(())
///     }
/// }
///
/// let mut registry = EventHandlerRegistry::new();
/// registry.register::<ProductCreatedEvent>(LogHandler);
/// registry.register::<ProductCreatedEvent>(ProjectionHandler);
///
/// let bus = EventBus::new(registry);
/// bus.dispatch(ProductCreatedEvent { id: 1, name: "Keyboard".into() }).await.unwrap();
/// # });
/// ```
#[derive(Clone, Debug)]
pub struct EventBus {
    #[doc(hidden)]
    registry: Arc<EventHandlerRegistry>,
}

impl EventBus {
    /// Creates a new `EventBus` backed by the provided registry.
    pub fn new(registry: EventHandlerRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// Dispatches (publishes) an event to every registered handler for its type.
    ///
    /// Handlers are invoked sequentially in registration order. If a handler
    /// returns an error, processing stops and that error is returned.
    ///
    /// # Errors
    ///
    /// Returns the first handler error encountered (if any).
    ///
    /// # Example
    /// ```
    /// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    /// # rt.block_on(async {
    /// use qonduit::async_trait;
    /// use qonduit::event::{Event, EventHandler, EventBus};
    /// use qonduit::registry::EventHandlerRegistry;
    ///
    /// #[derive(Clone, Debug)]
    /// struct OrderPaidEvent { order_id: u64 }
    /// impl Event for OrderPaidEvent {}
    ///
    /// struct IndexHandler;
    /// #[async_trait]
    /// impl EventHandler<OrderPaidEvent> for IndexHandler {
    ///     async fn handle(&self, e: OrderPaidEvent)
    ///         -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///         # let _ = e;
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let mut registry = EventHandlerRegistry::new();
    /// registry.register::<OrderPaidEvent>(IndexHandler);
    /// let bus = EventBus::new(registry);
    /// bus.dispatch(OrderPaidEvent { order_id: 42 }).await.unwrap();
    /// # });
    /// ```
    pub async fn dispatch<E: Event>(&self, event: E) -> Result<(), Box<dyn Error + Send + Sync>> {
        for handler in self.registry.get_handlers::<E>() {
            handler.handle(event.clone()).await?;
        }
        Ok(())
    }
}
