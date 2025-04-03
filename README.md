# Qonduit

[![Latest version](https://img.shields.io/crates/v/qonduit.svg)](https://crates.io/crates/qonduit)
[![Documentation](https://docs.rs/qonduit/badge.svg)](https://docs.rs/qonduit)
[![Build Status](https://img.shields.io/github/actions/workflow/status/botylev/qonduit/ci.yml)](https://github.com/botylev/qonduit/actions)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/botylev/qonduit/blob/master/LICENSE-MIT)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/botylev/qonduit/blob/master/LICENSE-APACHE)

Qonduit is a Rust implementation of the Command Query Responsibility Segregation (CQRS) architectural pattern. This library offers a structured approach to separate state-changing operations (commands) from data retrieval operations (queries) in your applications.

## Features

- **Command Handling**: Easily define commands that change the state of your system.
- **Query Handling**: Define queries that retrieve data without modifying the state.
- **Handler Registration**: Register command and query handlers using convenient macros.
- **Async Support**: Fully asynchronous handling of commands and queries using `async_trait`.

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

## Documentation

- [API Documentation](https://docs.rs/qonduit)

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.