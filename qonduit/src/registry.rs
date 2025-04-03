//! The `registry` module provides storage and lookup mechanisms for command and query handlers.
//!
//! This module contains the `CommandHandlerRegistry` and `QueryHandlerRegistry` structures responsible for
//! storing associations between command/query types and their respective handlers. These registries are used
//! by the `CommandBus` and `QueryBus` to locate the correct handler for each operation.
//!
//! The internal `wrapper` submodule contains type-erased wrappers that enable the registries to work with
//! handlers of different command and query types.
//!
//! - [CommandHandlerRegistry]: Stores and retrieves command handlers.
//! - [QueryHandlerRegistry]: Stores and retrieves query handlers.

use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FormatterResult;
use std::sync::Arc;

use crate::command::Command;
use crate::command::CommandHandler;
use crate::query::Query;
use crate::query::QueryHandler;
use crate::registry::wrapper::CommandHandlerWrapper;
use crate::registry::wrapper::QueryHandlerWrapper;

/// The `CommandHandlerRegistry` manages associations between command types and their handlers.
///
/// It provides a type-safe way to register handlers for specific command types and retrieve them
/// when needed for command execution.
#[derive(Default)]
pub struct CommandHandlerRegistry {
    #[doc(hidden)]
    pub(crate) handlers: HashMap<TypeId, Arc<dyn CommandHandlerWrapper>>,
}

/// Implementation for `CommandHandlerRegistry`
impl CommandHandlerRegistry {
    /// Creates an empty registry for command handlers.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a command handler for a specific command type.
    ///
    /// # Arguments
    ///
    /// * `handler` - The handler implementation to register for command type `C`.
    ///
    /// When commands of type `C` are dispatched, the registered handler will be used.
    ///
    /// # Example
    ///
    /// ```
    /// # use qonduit::command::{Command, CommandHandler};
    /// # use qonduit::async_trait;
    /// # use qonduit::registry::CommandHandlerRegistry;
    /// #
    /// # #[derive(Debug)]
    /// # struct AddProductCommand;
    /// #
    /// # impl Command for AddProductCommand {
    /// #   type Response = ();
    /// #   type Error = std::io::Error;
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct AddProductCommandHandler;
    /// #
    /// # #[async_trait]
    /// # impl CommandHandler<AddProductCommand> for AddProductCommandHandler {
    /// #   async fn handle(&self, _command: AddProductCommand) -> Result<(), std::io::Error> {
    /// #     Ok(())
    /// #   }
    /// # }
    /// let mut registry = CommandHandlerRegistry::new();
    /// registry.register::<AddProductCommand>(AddProductCommandHandler { /* ... */ });
    /// # assert!(true);
    /// ```
    pub fn register<C: Command>(&mut self, handler: impl CommandHandler<C> + 'static) {
        self.handlers.insert(
            TypeId::of::<C>(),
            Arc::new(Box::new(handler) as Box<dyn CommandHandler<C>>),
        );
    }

    /// Retrieves the handler for a specific command type.
    ///
    /// # Returns
    ///
    /// An `Option` containing the command handler if one is registered for type `C`,
    /// or `None` if no handler is known for this command type.
    ///
    /// # Example
    ///
    /// ```
    /// # use qonduit::command::{Command, CommandHandler};
    /// # use qonduit::async_trait;
    /// # use qonduit::registry::CommandHandlerRegistry;
    /// #
    /// # #[derive(Debug)]
    /// # struct AddProductCommand;
    /// #
    /// # impl Command for AddProductCommand {
    /// #   type Response = ();
    /// #   type Error = std::io::Error;
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct AddProductCommandHandler;
    /// #
    /// # #[async_trait]
    /// # impl CommandHandler<AddProductCommand> for AddProductCommandHandler {
    /// #   async fn handle(&self, _command: AddProductCommand) -> Result<(), std::io::Error> {
    /// #     Ok(())
    /// #   }
    /// # }
    /// let mut registry = CommandHandlerRegistry::new();
    /// registry.register(AddProductCommandHandler { /* ... */ });
    ///
    /// let handler = registry.get_handler::<AddProductCommand>();
    /// assert!(handler.is_some());
    /// ```
    pub fn get_handler<C: Command>(&self) -> Option<Box<dyn CommandHandler<C>>> {
        self.handlers
            .get(&TypeId::of::<C>())
            .cloned()
            .map(|handler| Box::new(handler) as Box<dyn CommandHandler<C>>)
    }
}

/// Debug implementation for `CommandHandlerRegistry`
impl Debug for CommandHandlerRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        f.debug_struct("CommandHandlerRegistry").finish()
    }
}

/// The `QueryHandlerRegistry` manages associations between query types and their handlers.
///
/// It provides a type-safe way to register handlers for specific query types and retrieve them
/// when needed for data query operations.
#[derive(Default)]
pub struct QueryHandlerRegistry {
    #[doc(hidden)]
    pub(crate) handlers: HashMap<TypeId, Arc<dyn QueryHandlerWrapper>>,
}

/// Implementation for `QueryHandlerRegistry`.
impl QueryHandlerRegistry {
    /// Creates an empty registry for query handlers.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a query handler for a specific query type.
    ///
    /// # Arguments
    ///
    /// * `handler` - The handler implementation to register for query type `Q`.
    ///
    /// When queries of type `Q` are dispatched, the registered handler will be used.
    ///
    /// # Example
    ///
    /// ```
    /// # use qonduit::query::{Query, QueryHandler};
    /// # use qonduit::async_trait;
    /// # use qonduit::registry::QueryHandlerRegistry;
    /// #
    /// # #[derive(Debug)]
    /// # struct FindProductQuery;
    /// #
    /// # impl Query for FindProductQuery {
    /// #   type Response = String;
    /// #   type Error = std::io::Error;
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct FindProductQueryHandler;
    /// #
    /// # #[async_trait]
    /// # impl QueryHandler<FindProductQuery> for FindProductQueryHandler {
    /// #   async fn handle(&self, _query: FindProductQuery) -> Result<String, std::io::Error> {
    /// #     Ok("Wireless Mouse".to_string())
    /// #   }
    /// # }
    /// let mut registry = QueryHandlerRegistry::new();
    /// registry.register(FindProductQueryHandler);
    /// # assert!(true);
    /// ```
    pub fn register<Q: Query>(&mut self, handler: impl QueryHandler<Q> + 'static) {
        self.handlers.insert(
            TypeId::of::<Q>(),
            Arc::new(Box::new(handler) as Box<dyn QueryHandler<Q>>),
        );
    }

    /// Retrieves the handler for a specific query type.
    ///
    /// # Returns
    ///
    /// An `Option` containing the query handler if one is registered for type `Q`,
    /// or `None` if no handler is known for this query type.
    ///
    /// # Example
    ///
    /// ```
    /// # use qonduit::query::{Query, QueryHandler};
    /// # use qonduit::async_trait;
    /// # use qonduit::registry::QueryHandlerRegistry;
    /// #
    /// # #[derive(Debug)]
    /// # struct FindProductQuery;
    /// #
    /// # impl Query for FindProductQuery {
    /// #   type Response = String;
    /// #   type Error = std::io::Error;
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct FindProductQueryHandler;
    /// #
    /// # #[async_trait]
    /// # impl QueryHandler<FindProductQuery> for FindProductQueryHandler {
    /// #   async fn handle(&self, _query: FindProductQuery) -> Result<String, std::io::Error> {
    /// #     Ok("Wireless Mouse".to_string())
    /// #   }
    /// # }
    /// let mut registry = QueryHandlerRegistry::new();
    /// registry.register(FindProductQueryHandler);
    ///
    /// let handler = registry.get_handler::<FindProductQuery>();
    /// assert!(handler.is_some());
    /// ```
    pub fn get_handler<Q: Query>(&self) -> Option<Box<dyn QueryHandler<Q>>> {
        self.handlers
            .get(&TypeId::of::<Q>())
            .cloned()
            .map(|handler| Box::new(handler) as Box<dyn QueryHandler<Q>>)
    }
}

/// Debug implementation for `QueryHandlerRegistry`
impl Debug for QueryHandlerRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        f.debug_struct("QueryHandlerRegistry").finish()
    }
}

#[doc(hidden)]
mod wrapper {
    use std::any::Any;
    use std::sync::Arc;

    use crate::async_trait;
    use crate::command::Command;
    use crate::command::CommandHandler;
    use crate::query::Query;
    use crate::query::QueryHandler;

    #[async_trait]
    pub trait CommandHandlerWrapper: Send + Sync {
        async fn execute(&self, command: Box<dyn Any + Send>) -> Box<dyn Any + Send>;
    }

    #[async_trait]
    pub trait QueryHandlerWrapper: Send + Sync {
        async fn execute(&self, query: Box<dyn Any + Send>) -> Box<dyn Any + Send>;
    }

    #[async_trait]
    impl<C: Command> CommandHandlerWrapper for Box<dyn CommandHandler<C>> {
        async fn execute(&self, command: Box<dyn Any + Send>) -> Box<dyn Any + Send> {
            let command = *command
                .downcast::<C>()
                .expect("Cannot downcast command to correct type");
            let result = self.handle(command).await;
            Box::new(result) as Box<dyn Any + Send>
        }
    }

    #[async_trait]
    impl<Q: Query> QueryHandlerWrapper for Box<dyn QueryHandler<Q>> {
        async fn execute(&self, query: Box<dyn Any + Send>) -> Box<dyn Any + Send> {
            let query = *query
                .downcast::<Q>()
                .expect("Cannot downcast query to correct type");
            let result = self.handle(query).await;
            Box::new(result) as Box<dyn Any + Send>
        }
    }

    #[async_trait]
    impl<C: Command> CommandHandler<C> for Arc<dyn CommandHandlerWrapper> {
        async fn handle(&self, command: C) -> Result<C::Response, C::Error> {
            let result = self.execute(Box::new(command)).await;
            *result
                .downcast()
                .expect("Cannot downcast command response to correct type")
        }
    }

    #[async_trait]
    impl<Q: Query> QueryHandler<Q> for Arc<dyn QueryHandlerWrapper> {
        async fn handle(&self, query: Q) -> Result<Q::Response, Q::Error> {
            let result = self.execute(Box::new(query)).await;
            *result
                .downcast()
                .expect("Cannot downcast query response to correct type")
        }
    }
}
