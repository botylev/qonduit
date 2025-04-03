//! Macros for quickly setting up command and query processing buses.
//!
//! It includes macros to ease the creation of `CommandBus` and `QueryBus` instances along with their associated
//! handler registries. These macros simplify the process of registering multiple handlers at once.
//!
//! Key macros provided by this module:
//!
//! - [command_bus]: Instantiates a `CommandBus` with registered handlers.
//! - [command_registry]: Creates a `CommandHandlerRegistry` with registered handlers.
//! - [query_bus]: Instantiates a `QueryBus` with registered handlers.
//! - [query_registry]: Creates a `QueryHandlerRegistry` with registered handlers.

/// A macro for initializing a `CommandBus` instance.
///
/// This macro creates a `CommandBus` with command handlers already registered,
/// simplifying the initialization process.
///
/// # Usage
///
/// This macro can be used in two ways:
///
/// 1. **With only handler instances:**
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
/// #       // Create a new product.
/// #       Ok(1)
/// #   }
/// # }
/// use qonduit::command_bus;
///
/// let command_bus = command_bus! {
///    AddProductCommandHandler { /* ... */ },
/// };
/// #
/// #   let command = AddProductCommand {
/// #       sku: "KB-ERGO-01".to_string(),
/// #       price: 99.99,
/// #   };
/// #
/// #   let result = command_bus.dispatch(command).await;
/// #   match result {
/// #       Ok(product_id) => {
/// #           assert_eq!(product_id, 1);
/// #           println!("Product created with id: {}", product_id);
/// #       },
/// #       Err(err) => {
/// #           assert!(false);
/// #           eprintln!("Failed to create product: {:?}", err);
/// #       }
/// #   }
/// # });
/// ```
///
/// This approach relies on compiler type inference to determine the command types.
///
/// 2. **With explicit command-handler pairs:**
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
/// #       // Create a new product.
/// #       Ok(1)
/// #   }
/// # }
/// use qonduit::command_bus;
///
/// let command_bus = command_bus! {
///    AddProductCommand => AddProductCommandHandler { /* ... */ },
/// };
/// #
/// #   let command = AddProductCommand {
/// #       sku: "KB-ERGO-01".to_string(),
/// #       price: 99.99,
/// #   };
/// #
/// #   let result = command_bus.dispatch(command).await;
/// #   match result {
/// #       Ok(product_id) => {
/// #           assert_eq!(product_id, 1);
/// #           println!("Product created with id: {}", product_id);
/// #       },
/// #       Err(err) => {
/// #           assert!(false);
/// #           eprintln!("Failed to create product: {:?}", err);
/// #       }
/// #   }
/// # });
/// ```
/// This approach explicitly specifies which command type each handler is responsible for.
#[macro_export]
macro_rules! command_bus {
        () => {{
            qonduit::command::CommandBus::new(qonduit::registry::CommandHandlerRegistry::new())
        }};
        ($($handler:expr),*$(,)?) => {{
            let mut command_handler_registry = qonduit::registry::CommandHandlerRegistry::new();
            $(command_handler_registry.register($handler);)*
            qonduit::command::CommandBus::new(command_handler_registry)
        }};
        ($($command:ty => $handler:expr),*$(,)?) => {{
            let mut command_handler_registry = qonduit::registry::CommandHandlerRegistry::new();
            $(command_handler_registry.register::<$command>($handler);)*
            qonduit::command::CommandBus::new(command_handler_registry)
        }};
    }

/// A macro for initializing a `CommandHandlerRegistry` instance.
///
/// This macro creates a `CommandHandlerRegistry` with handlers already registered,
/// simplifying the initialization process.
///
/// # Usage
///
/// This macro can be used in two ways:
///
/// 1. **With only handler instances:**
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
/// #       // Create a new product.
/// #       Ok(1)
/// #   }
/// # }
/// use qonduit::command_registry;
///
/// let command_registry = command_registry! {
///    AddProductCommandHandler { /* ... */ },
/// };
/// # });
/// ```
///
/// This approach relies on compiler type inference to determine the command types.
///
/// 2. **With explicit command-handler pairs:**
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
/// #       // Create a new product.
/// #       Ok(1)
/// #   }
/// # }
/// use qonduit::command_registry;
///
/// let command_registry = command_registry! {
///    AddProductCommand => AddProductCommandHandler { /* ... */ },
/// };
/// # });
/// ```
/// This approach explicitly specifies which command type each handler is responsible for.
#[macro_export]
macro_rules! command_registry {
        () => {{
            qonduit::command::CommandHandlerRegistry::new()
        }};
        ($($handler:expr),*$(,)?) => {{
            let mut command_handler_registry = qonduit::registry::CommandHandlerRegistry::new();
            $(command_handler_registry.register($handler);)*
            command_handler_registry
        }};
        ($($command:ty => $handler:expr),*$(,)?) => {{
            let mut command_handler_registry = qonduit::registry::CommandHandlerRegistry::new();
            $(command_handler_registry.register::<$command>($handler);)*
            command_handler_registry
        }};
    }

/// A macro for initializing a `QueryBus` instance.
///
/// This macro creates a `QueryBus` with query handlers already registered,
/// simplifying the initialization process.
///
/// # Usage
///
/// This macro can be used in two ways:
///
/// 1. **With only handler instances:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use qonduit::query::Query;
/// # use qonduit::async_trait;
/// # use qonduit::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct FindProductQuery {
/// #     product_id: u64,
/// # }
/// #
/// # impl Query for FindProductQuery {
/// #     type Response = String;
/// #     type Error = std::io::Error;
/// # }
/// #
/// # struct FindProductQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<FindProductQuery> for FindProductQueryHandler {
/// #     async fn handle(&self, query: FindProductQuery) -> Result<String, std::io::Error> {
/// #         Ok("Wireless Mouse".to_string())
/// #     }
/// # }
/// use qonduit::query_bus;
///
/// let query_bus = query_bus! {
///     FindProductQueryHandler { /* ... */ },
/// };
/// #
/// #   let query = FindProductQuery { product_id: 1 };
/// #   let result = query_bus.dispatch(query).await;
/// #   match result {
/// #       Ok(name) => println!("Product name: {}", name),
/// #       Err(err) => eprintln!("Failed to find product: {:?}", err),
/// #   }
/// # });
/// ```
///
/// This approach relies on compiler type inference to determine the query types.
///
/// 2. **With explicit query-handler pairs:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use qonduit::query::Query;
/// # use qonduit::async_trait;
/// # use qonduit::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct FindProductQuery {
/// #     product_id: u64,
/// # }
/// #
/// # impl Query for FindProductQuery {
/// #     type Response = String;
/// #     type Error = std::io::Error;
/// # }
/// #
/// # struct FindProductQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<FindProductQuery> for FindProductQueryHandler {
/// #     async fn handle(&self, query: FindProductQuery) -> Result<String, std::io::Error> {
/// #         Ok("Wireless Mouse".to_string())
/// #     }
/// # }
/// use qonduit::query_bus;
///
/// let query_bus = query_bus! {
///     FindProductQuery => FindProductQueryHandler { /* ... */ },
/// };
/// #
/// #   let query = FindProductQuery { product_id: 1 };
/// #   let result = query_bus.dispatch(query).await;
/// #   match result {
/// #       Ok(name) => println!("Product name: {}", name),
/// #       Err(err) => eprintln!("Failed to find product: {:?}", err),
/// #   }
/// # });
/// ```
///
/// This approach explicitly specifies which query type each handler is responsible for.
#[macro_export]
macro_rules! query_bus {
        () => {{
            qonduit::query::QueryBus::new(qonduit::registry::QueryHandlerRegistry::new())
        }};
        ($($handler:expr),*$(,)?) => {{
            let mut query_handler_registry = qonduit::registry::QueryHandlerRegistry::new();
            $(query_handler_registry.register($handler);)*
            qonduit::query::QueryBus::new(query_handler_registry)
        }};
        ($($query:ty => $handler:expr),*$(,)?) => {{
            let mut query_handler_registry = qonduit::registry::QueryHandlerRegistry::new();
            $(query_handler_registry.register::<$query>($handler);)*
            qonduit::query::QueryBus::new(query_handler_registry)
        }};
    }

/// A macro for initializing a `QueryHandlerRegistry` instance.
///
/// This macro creates a `QueryHandlerRegistry` with handlers already registered,
/// simplifying the initialization process.
///
/// # Usage
///
/// This macro can be used in two ways:
///
/// 1. **With only handler instances:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use qonduit::query::Query;
/// # use qonduit::async_trait;
/// # use qonduit::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct FindProductQuery {
/// #     product_id: u64,
/// # }
/// #
/// # impl Query for FindProductQuery {
/// #     type Response = String;
/// #     type Error = std::io::Error;
/// # }
/// #
/// # struct FindProductQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<FindProductQuery> for FindProductQueryHandler {
/// #     async fn handle(&self, query: FindProductQuery) -> Result<String, std::io::Error> {
/// #         Ok("Wireless Mouse".to_string())
/// #     }
/// # }
/// use qonduit::query_registry;
///
/// let query_registry = query_registry! {
///     FindProductQueryHandler { /* ... */ },
/// };
/// # });
/// ```
///
/// This approach relies on compiler type inference to determine the query types.
///
/// 2. **With explicit query-handler pairs:**
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use qonduit::query::Query;
/// # use qonduit::async_trait;
/// # use qonduit::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct FindProductQuery {
/// #     product_id: u64,
/// # }
/// #
/// # impl Query for FindProductQuery {
/// #     type Response = String;
/// #     type Error = std::io::Error;
/// # }
/// #
/// # struct FindProductQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<FindProductQuery> for FindProductQueryHandler {
/// #     async fn handle(&self, query: FindProductQuery) -> Result<String, std::io::Error> {
/// #         Ok("Wireless Mouse".to_string())
/// #     }
/// # }
/// use qonduit::query_registry;
///
/// let query_registry = query_registry! {
///     FindProductQuery => FindProductQueryHandler { /* ... */ },
/// };
/// # });
/// ```
///
/// This approach explicitly specifies which query type each handler is responsible for.
#[macro_export]
macro_rules! query_registry {
        () => {{
            qonduit::query::QueryHandlerRegistry::new()
        }};
        ($($handler:expr),*$(,)?) => {{
            let mut query_handler_registry = qonduit::registry::QueryHandlerRegistry::new();
            $(query_handler_registry.register($handler);)*
            query_handler_registry
        }};
        ($($query:ty => $handler:expr),*$(,)?) => {{
            let mut query_handler_registry = qonduit::registry::QueryHandlerRegistry::new();
            $(query_handler_registry.register::<$query>($handler);)*
            query_handler_registry
        }};
    }
