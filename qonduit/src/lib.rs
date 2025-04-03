//! The `qonduit` crate provides a Rust implementation of the Command Query Responsibility Segregation (CQRS) pattern.
//!
//! # Command Query Responsibility Segregation (CQRS)
//!
//! CQRS is an architectural pattern that separates read operations (queries) from write operations (commands).
//!
//! In this pattern, the `CommandBus` handles operations that modify system state, while the `QueryBus` manages
//! operations that retrieve data without making changes.
//!
//! Commands are instructions to perform actions that change the system state, whereas queries request information
//! without causing modifications.
//!
//! Both buses ensure that operations are processed by their appropriate handlers, which are registered in handler
//! registries. This separation promotes a more decoupled, maintainable, and scalable architecture.
//!
//! - [CommandBus](command::CommandBus): Routes commands to their designated handlers.
//! - [QueryBus](query::QueryBus): Routes queries to their designated handlers.
//!
//! # Example: Handling Commands
//!
//! This example shows how to define and handle commands in a CQRS-based system.
//! We'll create an `AddProductCommand` that adds a new product to inventory.
//! The `AddProductCommandHandler` processes this command and returns the ID of the newly added product.
//!
//! ```rust
//! use qonduit::async_trait;
//! use qonduit::command::Command;
//! use qonduit::command::CommandHandler;
//! use qonduit::command::CommandBus;
//! use qonduit::registry::CommandHandlerRegistry;
//!
//! // Define the AddProductCommand
//! #[derive(Debug)]
//! struct AddProductCommand {
//!     name: String,
//!     price: f64,
//! }
//!
//! // Define possible errors for the AddProductCommand
//! #[derive(Debug)]
//! enum AddProductError {
//!     ProductAlreadyExists,
//!     InvalidPrice,
//! }
//!
//! // Implement the Command trait for AddProductCommand
//! impl Command for AddProductCommand {
//!     type Response = u64; // Return the ID of the added product as Response
//!     type Error = AddProductError;
//! }
//!
//! // Define a handler for the AddProductCommand
//! struct AddProductCommandHandler {
//!     // Add any dependencies needed by the handler
//! }
//!
//! #[async_trait]
//! impl CommandHandler<AddProductCommand> for AddProductCommandHandler {
//!     async fn handle(&self, _command: AddProductCommand) -> Result<u64, AddProductError> {
//!         # return Ok(0);
//!         // Handle command logic here, e.g., add a product, check for duplicates, etc.
//!         todo!("Implement product addition logic");
//!     }
//! }
//!
//! # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
//! # rt.block_on(async {
//! // Create a command bus with the AddProductCommand and its handler
//!
//! // First, create a registry which stores mappings between command types and handlers
//! // The registry is a key component of the CQRS pattern that enables loose coupling
//! let mut registry = CommandHandlerRegistry::new();
//!
//! // Register our command handler for the AddProductCommand type
//! // This establishes the connection between command and handler in the registry
//! registry.register::<AddProductCommand>(AddProductCommandHandler { /* ... */ });
//!
//! // Create a command bus that uses our registry for dispatching commands
//! // The command bus is the user-facing API for executing commands in the system
//! let command_bus = CommandBus::new(registry);
//!
//! // Create a command to add a new product
//! let command = AddProductCommand {
//!     name: "Ergonomic Keyboard".to_string(),
//!     price: 59.99,
//! };
//!
//! // Dispatch the command through the bus.
//! // The bus looks up the appropriate handler in the registry and executes it
//! match command_bus.dispatch(command).await {
//!     Ok(product_id) => println!("Product added with ID: {}", product_id),
//!     Err(err) => println!("Failed to add product: {:?}", err),
//! }
//! # });
//! ```
//!
//! # Example: Handling Queries
//!
//! This example demonstrates how to define and handle queries in a CQRS-based system.
//! We'll create a `GetProductQuery` that retrieves product information by product ID.
//! The `GetProductQueryHandler` processes this query and returns the product's information if found.
//!
//! ```rust
//! use qonduit::async_trait;
//! use qonduit::query::Query;
//! use qonduit::query::QueryHandler;
//! use qonduit::query::QueryBus;
//! use qonduit::registry::QueryHandlerRegistry;
//!
//! // Define the GetProductQuery
//! #[derive(Debug)]
//! struct GetProductQuery {
//!     product_id: u64,
//! }
//!
//! // Define possible errors for the GetProductQuery
//! #[derive(Debug)]
//! enum GetProductError {
//!     ProductNotFound,
//! }
//!
//! // Define a Product struct
//! #[derive(Debug)]
//! struct Product {
//!     id: u64,
//!     name: String,
//!     price: f64,
//! }
//!
//! // Implement the Query trait for GetProductQuery
//! impl Query for GetProductQuery {
//!     type Response = Product; // Return the product as Response
//!     type Error = GetProductError;
//! }
//!
//! // Define a handler for the GetProductQuery
//! struct GetProductQueryHandler;
//!
//! #[async_trait]
//! impl QueryHandler<GetProductQuery> for GetProductQueryHandler {
//!     async fn handle(&self, _query: GetProductQuery) -> Result<Product, GetProductError> {
//!         # return Ok(Product { id: 0, name: "".to_string(), price: 0.0 });
//!         // Handle query logic here, e.g., retrieve the product by ID
//!         todo!("Implement product retrieval logic");
//!     }
//! }
//!
//! # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
//! # rt.block_on(async {
//! // Create a query bus with the GetProductQuery and its handler
//!
//! // First, create a registry which stores mappings between query types and handlers
//! // The registry provides a way to look up the appropriate handler for each query type
//! let mut registry = QueryHandlerRegistry::new();
//!
//! // Register our query handler for the GetProductQuery type
//! // This tells the registry which handler should process queries of this type
//! registry.register::<GetProductQuery>(GetProductQueryHandler { /* ... */ });
//!
//! // Create a query bus that uses the registry for dispatching queries
//! // The query bus provides a clean API for executing read operations
//! let query_bus = QueryBus::new(registry);
//!
//! // Create a query to retrieve the product with ID 1
//! let query = GetProductQuery { product_id: 1 };
//!
//! // Dispatch the query through the bus.
//! // The bus will find the right handler in the registry and execute it with our query
//! match query_bus.dispatch(query).await {
//!     Ok(product) => println!("Product found: {:?}", product),
//!     Err(err) => println!("Failed to retrieve product: {:?}", err),
//! }
//! # });
//! ```

pub mod command;
#[cfg(feature = "macros")]
pub mod macros;
pub mod query;
pub mod registry;

/// Re-exports the `async_trait` crate.
///
/// This crate provides a procedural macro for defining async traits in Rust.
///
/// For more details on `#[async_trait]`, see [mod@async_trait]
pub use async_trait::async_trait;
