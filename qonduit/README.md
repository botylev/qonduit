# Qonduit

[![Latest version](https://img.shields.io/crates/v/qonduit.svg)](https://crates.io/crates/qonduit)
[![Documentation](https://docs.rs/qonduit/badge.svg)](https://docs.rs/qonduit)
[![Build Status](https://img.shields.io/github/actions/workflow/status/botylev/qonduit/ci.yml)](https://github.com/botylev/qonduit/actions)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/botylev/qonduit/blob/master/LICENSE-MIT)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/botylev/qonduit/blob/master/LICENSE-APACHE)

Qonduit is a Rust implementation of the Command Query Responsibility Segregation (CQRS) architectural pattern. This library offers a structured approach to separate state-changing operations (commands) from data retrieval operations (queries) in your applications.

## Features

- **Command Handling**: Define commands that change the state of your system.
- **Query Handling**: Retrieve data without mutating state.
- **Event Handling (Fan-out)**: Publish immutable domain events to multiple handlers (e.g. projections, notifications).
- **Handler Registration Macros**: `command_bus!`, `query_bus!`, `event_bus!`, and matching `*_registry!` helpers.
- **Async Support**: Fully asynchronous handling via `async_trait`.
- **Lightweight & Type-Safe**: Minimal abstractions over strongly typed handlers.

## Installation

Add `qonduit` to your `Cargo.toml`:

```toml
[dependencies]
qonduit = "0.1.0"
```

## Usage

Here's an example showing how to set up Qonduit to handle an `AddProductCommand` in an inventory system:

```rust
use qonduit::async_trait;
use qonduit::command::Command;
use qonduit::command::CommandHandler;
use qonduit::command_bus;

// Define a command to add a product to inventory
#[derive(Debug)]
struct AddProductCommand {
    name: String,
    price: f64,
    stock: u32,
}

// Define possible errors
#[derive(Debug)]
enum ProductError {
    InvalidPrice,
    DuplicateSku,
    OutOfStock,
}

// Define the command response
#[derive(Debug)]
struct ProductResponse {
    id: u64,
}

// Implement the Command trait
impl Command for AddProductCommand {
    type Response = ProductResponse;
    type Error = ProductError;
}

// Create a handler for processing the command
struct InventoryCommandHandler {
    // Dependencies would go here
    next_id: u64,
}

// Implement the command handling logic
#[async_trait]
impl CommandHandler<AddProductCommand> for InventoryCommandHandler {
    async fn handle(&self, command: AddProductCommand) -> Result<ProductResponse, ProductError> {
        // Validate the command
        if command.price <= 0.0 {
            return Err(ProductError::InvalidPrice);
        }
        
        // In a real app, you would persist the product here
        println!("Adding product: {} at ${:.2}", command.name, command.price);
        
        // Return the new product ID
        Ok(ProductResponse { id: self.next_id })
    }
}

#[tokio::main]
async fn main() {
    // Create the command bus with our handler
    let command_bus = command_bus! {
        AddProductCommand => InventoryCommandHandler {
            next_id: 1001,
        },
    };

    // Create a command
    let command = AddProductCommand {
        name: "Ergonomic Keyboard".to_string(),
        price: 89.99,
        stock: 10,
    };

    // Dispatch the command
    match command_bus.dispatch(command).await {
        Ok(response) => {
            println!("Product added with ID: {}", response.id);
        }
        Err(err) => {
            eprintln!("Failed to add product: {:?}", err);
        }
    }
}
```

## Event System

The event system lets you broadcast immutable domain events to multiple handlers (fan‑out).
Each handler receives a cloned copy of the event and executes sequentially.

Example:

```rust
use qonduit::async_trait;
use qonduit::event::{Event, EventHandler};
use qonduit::event_bus;

// Define an event
#[derive(Clone, Debug)]
struct ProductCreated {
    id: u64,
    name: String,
}
impl Event for ProductCreated {}

// First handler
struct LogHandler;
#[async_trait]
impl EventHandler<ProductCreated> for LogHandler {
    async fn handle(&self, e: ProductCreated)
        -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    {
        println!("[log] product created {}", e.id);
        Ok(())
    }
}

// Second handler
struct ProjectionHandler;
#[async_trait]
impl EventHandler<ProductCreated> for ProjectionHandler {
    async fn handle(&self, e: ProductCreated)
        -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    {
        println!("[projection] updating read model for {}", e.id);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build an EventBus with two handlers for the same event type
    let bus = event_bus! {
        ProductCreated => LogHandler,
        ProductCreated => ProjectionHandler,
    };

    bus.dispatch(ProductCreated { id: 1, name: "Keyboard".into() }).await?;
    Ok(())
}
```

See the `examples/event.rs` example for a more complete version (including manual registry construction).

## Documentation

- [API Documentation](https://docs.rs/qonduit)

## License

Licensed under either of

* Apache License, Version 2.0
  (http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  (http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.