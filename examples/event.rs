//! Example demonstrating the event system with multiple handlers (fan-out).
//!
//! Run with:
//!    cargo run --example event
//!
//! This shows two equivalent construction styles:
//! 1. Using the `event_bus!` macro directly
//! 2. Manually creating an `EventHandlerRegistry` (commented out section)
//!
//! Each handler receives a cloned copy of the dispatched event.

use qonduit::async_trait;
use qonduit::event::{Event, EventHandler};
use qonduit::event_bus;
use std::error::Error;

// Domain event representing that a product was created.
#[derive(Clone, Debug)]
struct ProductCreatedEvent {
    id: u64,
    name: String,
}

// Implement the marker Event trait.
impl Event for ProductCreatedEvent {}

// First handler: logs the event.
struct LoggingHandler;

#[async_trait]
impl EventHandler<ProductCreatedEvent> for LoggingHandler {
    async fn handle(&self, event: ProductCreatedEvent) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("[log] product created: {} ({})", event.id, event.name);
        Ok(())
    }
}

// Second handler: simulates updating a projection / read model.
struct ProjectionHandler;

#[async_trait]
impl EventHandler<ProductCreatedEvent> for ProjectionHandler {
    async fn handle(&self, event: ProductCreatedEvent) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!(
            "[projection] updating read model with product {} ({})",
            event.id, event.name
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Build an EventBus using the macro (explicit mapping style).
    let event_bus = event_bus! {
        ProductCreatedEvent => LoggingHandler,
        ProductCreatedEvent => ProjectionHandler,
    };

    // Alternative manual construction (uncomment to use):
    /*
    let mut registry = EventHandlerRegistry::new();
    registry.register::<ProductCreatedEvent>(LoggingHandler);
    registry.register::<ProductCreatedEvent>(ProjectionHandler);
    let event_bus = EventBus::new(registry);
    */

    // Dispatch an event; both handlers will run in registration order.
    event_bus
        .dispatch(ProductCreatedEvent {
            id: 1001,
            name: "Ergonomic Keyboard".to_string(),
        })
        .await?;

    Ok(())
}
