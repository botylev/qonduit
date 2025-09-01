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
use crate::event::{Event, EventHandler};
use crate::query::Query;
use crate::query::QueryHandler;
use crate::registry::wrapper::QueryHandlerWrapper;
use crate::registry::wrapper::{CommandHandlerWrapper, EventHandlerWrapper};

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

#[derive(Default)]
pub struct EventHandlerRegistry {
    #[doc(hidden)]
    pub(crate) handlers: HashMap<TypeId, Vec<Arc<dyn EventHandlerWrapper>>>,
}

/// A registry that stores lists of event handlers keyed by concrete event type.
///
/// Unlike command/query registries (which hold a single handler per type),
/// the event registry supports *fan‑out*: multiple handlers can be registered
/// for the same event type. When an event is dispatched, each registered handler
/// is invoked sequentially.
///
/// Typical usage is to:
/// 1. Create a registry
/// 2. Register one or more handlers for each event type
/// 3. Construct an [`EventBus`](crate::event::EventBus) with the registry
///
/// # Example
/// ```
/// use qonduit::async_trait;
/// use qonduit::event::{Event, EventHandler, EventBus};
/// use qonduit::registry::EventHandlerRegistry;
///
/// #[derive(Clone, Debug)]
/// struct UserRegisteredEvent { user_id: u64 }
/// impl Event for UserRegisteredEvent {}
///
/// struct SendWelcomeEmail;
/// struct UpdateProjection;
///
/// #[async_trait]
/// impl EventHandler<UserRegisteredEvent> for SendWelcomeEmail {
///     async fn handle(&self, _e: UserRegisteredEvent)
///         -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///         // Send a welcome email
///         Ok(())
///     }
/// }
///
/// #[async_trait]
/// impl EventHandler<UserRegisteredEvent> for UpdateProjection {
///     async fn handle(&self, _e: UserRegisteredEvent)
///         -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///         // Update read model / projection
///         Ok(())
///     }
/// }
///
/// let mut reg = EventHandlerRegistry::new();
/// reg.register::<UserRegisteredEvent>(SendWelcomeEmail);
/// reg.register::<UserRegisteredEvent>(UpdateProjection);
///
/// let bus = EventBus::new(reg);
/// # drop(bus);
/// ```
impl EventHandlerRegistry {
    /// Creates an empty event handler registry.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers an event handler for the event type `E`.
    ///
    /// Multiple handlers may be registered for the same event type; they will
    /// be invoked in the order of registration.
    ///
    /// # Type Parameters
    /// * `E` - Event type the handler reacts to.
    ///
    /// # Example
    /// ```
    /// # use qonduit::async_trait;
    /// # use qonduit::event::{Event, EventHandler};
    /// # use qonduit::registry::EventHandlerRegistry;
    /// #[derive(Clone, Debug)]
    /// struct SomethingHappened;
    /// impl Event for SomethingHappened {}
    ///
    /// struct Audit;
    ///
    /// #[async_trait]
    /// impl EventHandler<SomethingHappened> for Audit {
    ///     async fn handle(&self, _e: SomethingHappened)
    ///         -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let mut registry = EventHandlerRegistry::new();
    /// registry.register::<SomethingHappened>(Audit);
    /// ```
    pub fn register<E: Event>(&mut self, handler: impl EventHandler<E> + 'static) {
        let handlers = self.handlers.entry(TypeId::of::<E>()).or_default();
        handlers.push(Arc::new(Box::new(handler) as Box<dyn EventHandler<E>>));
    }

    /// Returns all handlers registered for the event type `E`.
    ///
    /// If no handlers are registered for `E`, an empty vector is returned.
    pub fn get_handlers<E: Event>(&self) -> Vec<Arc<dyn EventHandler<E>>> {
        self.handlers
            .get(&TypeId::of::<E>())
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|handler| Arc::new(handler) as Arc<dyn EventHandler<E>>)
            .collect()
    }
}

impl Debug for EventHandlerRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        f.debug_struct("EventHandlerRegistry").finish()
    }
}

#[doc(hidden)]
mod wrapper {
    //! Internal type-erasure layer.
    //!
    //! The public registries store handlers behind trait objects without
    //! exposing their generic type parameters. To achieve this, we:
    //! 1. Box each concrete handler (e.g. `Box<dyn CommandHandler<C>>`)
    //! 2. Upcast it to a type‑erased wrapper trait object (`CommandHandlerWrapper`)
    //! 3. On dispatch, pass boxed `dyn Any` values, then downcast back to the
    //!    concrete command/query/event type.
    //!
    //! This allows storing heterogeneous handlers in homogeneous maps keyed
    //! by `TypeId` while keeping the outer API strongly typed.

    use std::any::Any;
    use std::error::Error;
    use std::sync::Arc;

    use crate::async_trait;
    use crate::command::Command;
    use crate::command::CommandHandler;
    use crate::event::{Event, EventHandler};
    use crate::query::Query;
    use crate::query::QueryHandler;

    /// Type-erased asynchronous executor for a concrete `CommandHandler<C>`.
    #[async_trait]
    pub trait CommandHandlerWrapper: Send + Sync {
        /// Accepts a boxed `Any` value that must be a `C`, executes the handler,
        /// and returns the boxed result (`Result<C::Response, C::Error>` as `Any`).
        async fn execute(&self, command: Box<dyn Any + Send>) -> Box<dyn Any + Send>;
    }

    /// Type-erased asynchronous executor for a concrete `QueryHandler<Q>`.
    #[async_trait]
    pub trait QueryHandlerWrapper: Send + Sync {
        async fn execute(&self, query: Box<dyn Any + Send>) -> Box<dyn Any + Send>;
    }

    /// Type-erased asynchronous executor for a concrete `EventHandler<E>`.
    #[async_trait]
    pub trait EventHandlerWrapper: Send + Sync {
        async fn execute(&self, event: Box<dyn Any + Send>) -> Box<dyn Any + Send>;
    }

    // -----------------------
    // Concrete -> Wrapper impl
    // -----------------------

    #[async_trait]
    impl<C: Command> CommandHandlerWrapper for Box<dyn CommandHandler<C>> {
        async fn execute(&self, command: Box<dyn Any + Send>) -> Box<dyn Any + Send> {
            // Downcast the erased box to the concrete command type.
            let command = *command
                .downcast::<C>()
                .expect("Cannot downcast command to correct type");
            // Delegate to the real handler.
            let result = self.handle(command).await;
            // Re-box the result for the outer layer.
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
    impl<E: Event> EventHandlerWrapper for Box<dyn EventHandler<E>> {
        async fn execute(&self, event: Box<dyn Any + Send>) -> Box<dyn Any + Send> {
            let event = *event
                .downcast::<E>()
                .expect("Cannot downcast event to correct type");
            let result = self.handle(event).await;
            Box::new(result) as Box<dyn Any + Send>
        }
    }

    // -------------------------------------------
    // Wrapper trait objects re‑implement handlers
    // -------------------------------------------
    //
    // We now implement the *public* handler traits for `Arc<dyn *_Wrapper>` so the
    // registries can hand out objects that still satisfy `CommandHandler<C>`,
    // `QueryHandler<Q>`, or `EventHandler<E>` while internally delegating through
    // the erased wrapper interface.

    #[async_trait]
    impl<C: Command> CommandHandler<C> for Arc<dyn CommandHandlerWrapper> {
        async fn handle(&self, command: C) -> Result<C::Response, C::Error> {
            let result_any = self.execute(Box::new(command)).await;
            // The inner boxed value is `Result<C::Response, C::Error>` stored as Any.
            *result_any
                .downcast()
                .expect("Cannot downcast command response to correct type")
        }
    }

    #[async_trait]
    impl<Q: Query> QueryHandler<Q> for Arc<dyn QueryHandlerWrapper> {
        async fn handle(&self, query: Q) -> Result<Q::Response, Q::Error> {
            let result_any = self.execute(Box::new(query)).await;
            *result_any
                .downcast()
                .expect("Cannot downcast query response to correct type")
        }
    }

    #[async_trait]
    impl<E: Event> EventHandler<E> for Arc<dyn EventHandlerWrapper> {
        async fn handle(&self, event: E) -> Result<(), Box<dyn Error + Send + Sync>> {
            let result_any = self.execute(Box::new(event)).await;
            *result_any
                .downcast()
                .expect("Cannot downcast event handle result to correct type")
        }
    }
}
