//! The `command` module provides essential abstractions and components for state-changing operations in the CQRS architecture.
//!
//! Commands are instructions that modify system state. This module defines the `Command` trait which
//! represents these modification requests, and the `CommandHandler` trait which implements the logic for
//! processing them.
//!
//! The `CommandBus` facilitates command execution by dispatching requests to their appropriate handlers. It uses the
//! `CommandHandlerRegistry` from the [registry](crate::registry) module to track available handlers.
//!
//! - [Command]: Represents a command trait in the system.
//! - [CommandHandler]: Represents a trait for handling commands.
//! - [CommandBus]: Dispatches commands to their appropriate handlers.
//!
//! # See Also
//!
//! - [CommandHandlerRegistry]: Manages the collection of command handlers.

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

use crate::async_trait;
use crate::registry::CommandHandlerRegistry;

/// The `Command` trait defines an operation that modifies the system state.
///
/// Commands enable structured state changes while maintaining type safety.
///
/// # Example
///
/// ```
/// use qonduit::command::Command;
///
/// #[derive(Debug)]
/// enum AddProductError {
///    SkuAlreadyExists,
///    PriceIsNegative,
/// }
///
/// #[derive(Debug)]
/// struct AddProductCommand {
///    sku: String,
///    price: f64,
/// }
///
/// impl Command for AddProductCommand {
///   // The identifier of the newly added product.
///   type Response = u64;
///   // The error type that is returned if the command fails.
///   type Error = AddProductError;
/// }
/// ```
pub trait Command: Send + Sync + Any + Debug {
    /// The response type returned when the command succeeds.
    ///
    /// This type must implement the `Debug`, `Send`, and `Sync` traits.
    type Response: Debug + Send + Sync;

    /// The error type returned when the command fails.
    ///
    /// This type must implement the `Debug`, `Send`, and `Sync` traits.
    type Error: Debug + Send + Sync;
}

/// The `CommandHandler` trait represents a handler that processes a command.
///
/// # Example
///
/// ```
/// # use qonduit::command::Command;
/// #
/// # #[derive(Debug)]
/// # enum AddProductError {
/// #    SkuAlreadyExists,
/// #    PriceIsNegative,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct AddProductCommand {
/// #    sku: String,
/// #    price: f64,
/// # }
/// #
/// # impl Command for AddProductCommand {
/// #   // The identifier of the newly added product.
/// #   type Response = u64;
/// #   // The error type that is returned if the command fails.
/// #   type Error = AddProductError;
/// # }
/// use qonduit::async_trait;
/// use qonduit::command::CommandHandler;
///
/// struct AddProductCommandHandler;
///
/// #[async_trait]
/// impl CommandHandler<AddProductCommand> for AddProductCommandHandler {
///    async fn handle(&self, command: AddProductCommand) -> Result<u64, AddProductError> {
///       // Add a new product to the system.
///       Ok(42)
///   }
/// }
/// ```
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    /// Executes the command processing logic.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to be processed.
    ///
    /// # Returns
    ///
    /// Either the successful response of the command execution or an error if it fails.
    async fn handle(&self, command: C) -> Result<C::Response, C::Error>;
}

/// The `CommandBus` dispatches commands to their corresponding handlers for processing.
///
/// It serves as the central coordinator for all state-changing operations.
///
/// # Example
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use qonduit::command::Command;
/// # use qonduit::async_trait;
/// # use qonduit::command::CommandHandler;
/// #
/// # #[derive(Debug)]
/// # enum AddProductError {
/// #    SkuAlreadyExists,
/// #    PriceIsNegative,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct AddProductCommand {
/// #    sku: String,
/// #    price: f64,
/// # }
/// #
/// # impl Command for AddProductCommand {
/// #   // The identifier of the newly added product.
/// #   type Response = u64;
/// #   // The error type that is returned if the command fails.
/// #   type Error = AddProductError;
/// # }
/// #
/// # struct AddProductCommandHandler;
/// #
/// # #[async_trait]
/// # impl CommandHandler<AddProductCommand> for AddProductCommandHandler {
/// #    async fn handle(&self, command: AddProductCommand) -> Result<u64, AddProductError> {
/// #       // Add a new product to the system.
/// #       Ok(42)
/// #   }
/// # }
/// use qonduit::registry::CommandHandlerRegistry;
/// use qonduit::command::CommandBus;
///
/// // First, create a registry to store our command handlers
/// // The registry is responsible for managing mappings between command types and their handlers
/// let mut registry = CommandHandlerRegistry::new();
///
/// // Register a handler for AddProductCommand
/// // This tells the registry which handler should process commands of this type
/// registry.register::<AddProductCommand>(AddProductCommandHandler { /* ... */ });
///
/// // Create a command bus that will use this registry to dispatch commands
/// // The bus serves as the entry point for all command operations in the system
/// let command_bus = CommandBus::new(registry);
///
/// let command = AddProductCommand {
///   sku: "KB-ERGO-01".to_string(),
///   price: 129.99,
/// };
///
/// // When we dispatch a command, the bus will:
/// // 1. Find the right handler in the registry based on the command's type
/// // 2. Execute the handler with our command
/// // 3. Return the result
/// let result = command_bus.dispatch(command).await;
/// match result {
///     Ok(product_id) => {
///         # assert_eq!(product_id, 42);
///         println!("Product added with id: {}", product_id);
///     },
///     Err(err) => {
///         # assert!(false);
///         eprintln!("Failed to add product: {:?}", err);
///     }
/// }
/// # });
/// ```
#[derive(Clone, Debug)]
pub struct CommandBus {
    #[doc(hidden)]
    registry: Arc<CommandHandlerRegistry>,
}

/// Implementation of the `CommandBus`.
impl CommandBus {
    /// Creates a new instance of the `CommandBus`.
    ///
    /// Initializes a command bus with a registry of handlers that will
    /// process commands.
    ///
    /// # Arguments
    ///
    /// * `registry` - The registry containing command handlers.
    ///
    /// # Example
    ///
    /// ```
    /// use qonduit::command::CommandBus;
    /// use qonduit::registry::CommandHandlerRegistry;
    ///
    /// let registry = CommandHandlerRegistry::new();
    /// let command_bus = CommandBus::new(registry);
    ///
    /// # assert!(true);
    /// ```
    pub fn new(registry: CommandHandlerRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// Dispatches a command to its corresponding handler and returns the result of the command execution.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to be executed.
    ///
    /// # Returns
    ///
    /// The result from the command handler, either containing the response data of the command execution or an error.
    ///
    /// # Panics
    ///
    /// This method will panic if no handler is registered for the command type.
    ///
    /// # Example
    ///
    /// ```
    /// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    /// # rt.block_on(async {
    /// # use qonduit::command::Command;
    /// # use qonduit::async_trait;
    /// # use qonduit::command::CommandHandler;
    /// #
    /// # #[derive(Debug)]
    /// # enum AddProductError {
    /// #    SkuAlreadyExists,
    /// #    PriceIsNegative,
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct AddProductCommand {
    /// #    sku: String,
    /// #    price: f64,
    /// # }
    /// #
    /// # impl Command for AddProductCommand {
    /// #   // The identifier of the newly added product.
    /// #   type Response = u64;
    /// #   // The error type that is returned if the command fails.
    /// #   type Error = AddProductError;
    /// # }
    /// #
    /// # struct AddProductCommandHandler;
    /// #
    /// # #[async_trait]
    /// # impl CommandHandler<AddProductCommand> for AddProductCommandHandler {
    /// #    async fn handle(&self, command: AddProductCommand) -> Result<u64, AddProductError> {
    /// #       // Add a new product to the system.
    /// #       Ok(42)
    /// #   }
    /// # }
    /// use qonduit::registry::CommandHandlerRegistry;
    /// use qonduit::command::CommandBus;
    ///
    /// // Set up the command handler registry
    /// // The registry stores associations between command types and their handlers
    /// let mut registry = CommandHandlerRegistry::new();
    ///
    /// // Register our AddProductCommand with its handler
    /// // This creates a type-based mapping in the registry
    /// registry.register::<AddProductCommand>(AddProductCommandHandler { /* ... */ });
    ///
    /// // Create the CommandBus using our registry
    /// // The bus will use this registry to look up the appropriate handler for each command
    /// let command_bus = CommandBus::new(registry);
    ///
    /// let command = AddProductCommand {
    ///   sku: "KB-ERGO-01".to_string(),
    ///   price: 129.99,
    /// };
    ///
    /// // The dispatch method:
    /// // 1. Uses the command's type to find the appropriate handler in the registry
    /// // 2. Calls the handler's handle method with the command
    /// // 3. Returns the handler's result
    /// let result = command_bus.dispatch(command).await;
    /// match result {
    ///     Ok(product_id) => {
    ///         # assert_eq!(product_id, 42);
    ///         println!("Product added with id: {}", product_id);
    ///     },
    ///     Err(err) => {
    ///         # assert!(false);
    ///         eprintln!("Failed to add product: {:?}", err);
    ///     }
    /// }
    /// # });
    /// ```
    pub async fn dispatch<C: Command>(&self, command: C) -> Result<C::Response, C::Error> {
        match self.registry.get_handler::<C>() {
            None => {
                panic!(
                    "No handler registered for command: {:?}",
                    std::any::type_name::<C>()
                );
            }
            Some(handler) => handler.handle(command).await,
        }
    }
}
