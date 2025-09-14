use qonduit::async_trait;
use qonduit::event::{Event, EventBus, EventHandler};
use qonduit::registry::EventHandlerRegistry;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::SeqCst;

#[derive(Debug, Clone)]
struct TestEvent();
impl Event for TestEvent {}

struct TestEventHandler;

#[async_trait]
impl EventHandler<TestEvent> for TestEventHandler {
    async fn handle(&self, _event: TestEvent) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}

#[tokio::test]
async fn test_event_bus_successful_dispatch() {
    // Create registry and register handler
    let mut registry = EventHandlerRegistry::new();
    registry.register::<TestEvent>(TestEventHandler);

    // Create bus with registry
    let bus = EventBus::new(registry);

    // Dispatch event
    let result = bus.dispatch(TestEvent()).await;

    // Check result
    assert!(result.is_ok());
}

// ===== Event Registry Tests =====

// Event type 1 for registry tests
#[derive(Debug, Clone)]
struct Event1();

impl Event for Event1 {}

struct Event1Handler;

#[async_trait]
impl EventHandler<Event1> for Event1Handler {
    async fn handle(&self, _event: Event1) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}

// Event type 2 for registry tests
#[derive(Debug, Clone)]
struct Event2();

impl Event for Event2 {}

struct Event2Handler;

#[async_trait]
impl EventHandler<Event2> for Event2Handler {
    async fn handle(&self, _event: Event2) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}

#[tokio::test]
async fn test_event_registry_empty() {
    let registry = EventHandlerRegistry::new();
    assert!(registry.get_handlers::<Event1>().is_empty());
}

#[tokio::test]
async fn test_event_registry_single_registration() {
    let mut registry = EventHandlerRegistry::new();

    // Initially no handler is registered
    assert!(registry.get_handlers::<Event1>().is_empty());

    // Register a handler
    registry.register::<Event1>(Event1Handler);

    // Now a handler should be available
    let handler = registry.get_handlers::<Event1>();
    assert!(!handler.is_empty());

    // The handler should work correctly
    let result = handler.get(0).unwrap().handle(Event1()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_event_registry_multiple_registrations() {
    let mut registry = EventHandlerRegistry::new();

    // Register multiple handlers
    registry.register::<Event1>(Event1Handler);
    registry.register::<Event2>(Event2Handler);

    // Both handlers should be available
    assert!(!registry.get_handlers::<Event1>().is_empty());
    assert!(!registry.get_handlers::<Event2>().is_empty());

    // Test Event1 handler
    let handlers1 = registry.get_handlers::<Event1>();
    let result1 = handlers1.get(0).unwrap().handle(Event1()).await;
    assert!(result1.is_ok());

    // Test Event2 handler
    let handlers2 = registry.get_handlers::<Event2>();
    let result2 = handlers2.get(0).unwrap().handle(Event2()).await;
    assert!(result2.is_ok());
}

#[derive(Debug, Clone)]
struct IncEvent(pub u32);
impl Event for IncEvent {}

struct IncEventHandler {
    pub counter: Arc<AtomicU32>,
}

#[async_trait]
impl EventHandler<IncEvent> for IncEventHandler {
    async fn handle(&self, event: IncEvent) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.counter.fetch_add(event.0, SeqCst);
        Ok(())
    }
}

#[tokio::test]
async fn test_event_bus_successful_dispatch_inc_single() {
    let counter = Arc::new(AtomicU32::new(0));
    let handler = IncEventHandler {
        counter: counter.clone(),
    };
    // Create registry and register handler
    let mut registry = EventHandlerRegistry::new();
    registry.register::<IncEvent>(handler);

    // Create bus with registry
    let bus = EventBus::new(registry);

    // Dispatch event
    let result = bus.dispatch(IncEvent(1)).await;

    // Check result
    assert!(result.is_ok());
    assert_eq!(counter.load(SeqCst), 1);

    let result = bus.dispatch(IncEvent(2)).await;

    // Check result
    assert!(result.is_ok());
    assert_eq!(counter.load(SeqCst), 3);
}

#[tokio::test]
async fn test_event_bus_successful_dispatch_inc_multiple() {
    let counter = Arc::new(AtomicU32::new(0));
    let handler1 = IncEventHandler {
        counter: counter.clone(),
    };
    let handler2 = IncEventHandler {
        counter: counter.clone(),
    };
    // Create registry and register handler
    let mut registry = EventHandlerRegistry::new();
    registry.register::<IncEvent>(handler1);
    registry.register::<IncEvent>(handler2);

    // Create bus with registry
    let bus = EventBus::new(registry);

    // Dispatch event
    let result = bus.dispatch(IncEvent(1)).await;

    // Check result
    assert!(result.is_ok());
    assert_eq!(counter.load(SeqCst), 2);

    let result = bus.dispatch(IncEvent(2)).await;

    // Check result
    assert!(result.is_ok());
    assert_eq!(counter.load(SeqCst), 6);
}
