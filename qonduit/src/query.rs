//! The `query` module contains fundamental abstractions and components for handling data retrieval in the CQRS architecture.
//!
//! Queries are operations that fetch data without altering the system's state. This module defines the `Query` trait
//! which represents a data retrieval request, and the `QueryHandler` trait which implements the logic for processing
//! these requests.
//!
//! The `QueryBus` coordinates query execution by routing them to the appropriate handlers. It works with the
//! `QueryHandlerRegistry` from the [registry](crate::registry) module to track available handlers.
//!
//! - [Query]: Represents a query trait in the system.
//! - [QueryHandler]: Represents a trait for handling queries.
//! - [QueryBus]: Routes queries to their appropriate handlers.
//!
//! # See Also
//!
//! - [QueryHandlerRegistry]: Manages the collection of query handlers.

use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

use crate::async_trait;
use crate::registry::QueryHandlerRegistry;

/// The `Query` trait defines a query for retrieving data from the system.
///
/// Queries allow to access information while maintaining system state intact.
///
/// # Example
///
/// ```
/// use qonduit::query::Query;
///
/// #[derive(Debug)]
/// struct Product {
///     id: u64,
///     name: String,
///     price: f64,
/// }
///
/// #[derive(Debug)]
/// enum FindProductError {
///     ProductNotFound,
///     InvalidQuery,
/// }
///
/// #[derive(Debug)]
/// struct FindProductQuery {
///    product_id: u64,
/// }
///
/// impl Query for FindProductQuery {
///   // The data returned by the query - a Product
///   type Response = Product;
///   // Possible errors when executing the query
///   type Error = FindProductError;
/// }
/// ```
pub trait Query: Send + Sync + Any + Debug {
    /// The response type returned by the query.
    ///
    /// This type must implement the `Send` and `Sync` traits.
    type Response: Send + Sync;

    /// The error type that can occur during query execution.
    ///
    /// This type must implement the `Debug`, `Send`, and `Sync` traits.
    type Error: Debug + Send + Sync;
}

/// The `QueryHandler` trait represents a handler that processes a query.
///
/// # Example
///
/// ```
/// # use qonduit::query::Query;
/// # use qonduit::async_trait;
/// #
/// # #[derive(Debug)]
/// # struct Product {
/// #     id: u64,
/// #     name: String,
/// #     price: f64,
/// # }
/// #
/// # #[derive(Debug)]
/// # enum FindProductError {
/// #     ProductNotFound,
/// #     InvalidQuery,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct FindProductQuery {
/// #    product_id: u64,
/// # }
/// #
/// # impl Query for FindProductQuery {
/// #   type Response = Product;
/// #   type Error = FindProductError;
/// # }
/// #
/// use qonduit::query::QueryHandler;
///
/// struct FindProductQueryHandler;
///
/// #[async_trait]
/// impl QueryHandler<FindProductQuery> for FindProductQueryHandler {
///    async fn handle(&self, query: FindProductQuery) -> Result<Product, FindProductError> {
///       // Query the data store for product information
/// #     Ok(Product {
/// #         id: query.product_id,
/// #         name: "Wireless Mouse".to_string(),
/// #         price: 29.99,
/// #     })
///   }
/// }
/// ```
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    /// Executes the query processing logic.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to be processed.
    ///
    /// # Returns
    ///
    /// Either the requested data or an error if the query execution fails.
    async fn handle(&self, query: Q) -> Result<Q::Response, Q::Error>;
}

/// The `QueryBus` dispatches queries to their corresponding handlers for processing.
///
/// It serves as the central coordinator for all query operations.
///
/// # Example
///
/// ```
/// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
/// # rt.block_on(async {
/// # use qonduit::query::Query;
/// # use qonduit::async_trait;
/// # use qonduit::query::QueryHandler;
/// #
/// # #[derive(Debug)]
/// # struct Product {
/// #     id: u64,
/// #     name: String,
/// #     price: f64,
/// # }
/// #
/// # #[derive(Debug)]
/// # enum FindProductError {
/// #     ProductNotFound,
/// #     InvalidQuery,
/// # }
/// #
/// # #[derive(Debug)]
/// # struct FindProductQuery {
/// #    product_id: u64,
/// # }
/// #
/// # impl Query for FindProductQuery {
/// #   type Response = Product;
/// #   type Error = FindProductError;
/// # }
/// #
/// # struct FindProductQueryHandler;
/// #
/// # #[async_trait]
/// # impl QueryHandler<FindProductQuery> for FindProductQueryHandler {
/// #    async fn handle(&self, query: FindProductQuery) -> Result<Product, FindProductError> {
/// #       // Retrieve product information
/// #       Ok(Product {
/// #           id: query.product_id,
/// #           name: "Wireless Mouse".to_string(),
/// #           price: 29.99,
/// #       })
/// #   }
/// # }
/// use qonduit::registry::QueryHandlerRegistry;
/// use qonduit::query::QueryBus;
///
/// // First, create a registry to store our query handlers
/// // The registry maintains a mapping of query types to their handlers
/// let mut registry = QueryHandlerRegistry::new();
///
/// // Register a handler for the FindProductQuery
/// // This tells the registry which handler should process queries of this type
/// registry.register::<FindProductQuery>(FindProductQueryHandler { /* ... */ });
///
/// // Create a query bus that will use this registry to dispatch queries
/// // The bus relies on the registry to look up the appropriate handler
/// let query_bus = QueryBus::new(registry);
///
/// let query = FindProductQuery {
///    product_id: 1,
/// };
///
/// // When we dispatch a query, the bus will:
/// // 1. Find the right handler in the registry
/// // 2. Execute the handler with our query
/// // 3. Return the result
/// let result = query_bus.dispatch(query).await;
/// match result {
///     Ok(product) => {
///         # assert_eq!(product.name, "Wireless Mouse".to_string());
///         println!("Product found: {} - ${:.2}", product.name, product.price);
///     },
///     Err(err) => {
///         # assert!(false);
///         match err {
///             FindProductError::ProductNotFound => println!("Product not found"),
///             FindProductError::InvalidQuery => println!("Invalid product query"),
///         }
///     }
/// }
/// # });
/// ```
#[derive(Clone, Debug)]
pub struct QueryBus {
    #[doc(hidden)]
    registry: Arc<QueryHandlerRegistry>,
}

/// Implementation of the `QueryBus`.
impl QueryBus {
    /// Creates a new instance of the `QueryBus`.
    ///
    /// Initializes a query bus with a registry of handlers
    /// that will process queries.
    ///
    /// # Arguments
    ///
    /// * `registry` - The registry containing query handlers.
    ///
    /// # Example
    ///
    /// ```
    /// use qonduit::query::QueryBus;
    /// use qonduit::registry::QueryHandlerRegistry;
    ///
    /// let registry = QueryHandlerRegistry::new();
    /// let query_bus = QueryBus::new(registry);
    ///
    /// # assert!(true);
    /// ```
    pub fn new(registry: QueryHandlerRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// Dispatches a query to its corresponding handler and returns the result.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to be dispatched.
    ///
    /// # Returns
    ///
    /// The result from the query handler, either containing the Response data or an error.
    ///
    /// # Panics
    ///
    /// This method will panic if no handler is registered for the query type.
    ///
    /// # Example
    ///
    /// ```
    /// # let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    /// # rt.block_on(async {
    /// # use qonduit::query::Query;
    /// # use qonduit::async_trait;
    /// # use qonduit::query::QueryHandler;
    /// #
    /// # #[derive(Debug)]
    /// # struct Product {
    /// #     id: u64,
    /// #     name: String,
    /// #     price: f64,
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # enum FindProductError {
    /// #     ProductNotFound,
    /// #     InvalidQuery,
    /// # }
    /// #
    /// # #[derive(Debug)]
    /// # struct FindProductQuery {
    /// #    product_id: u64,
    /// # }
    /// #
    /// # impl Query for FindProductQuery {
    /// #   type Response = Product;
    /// #   type Error = FindProductError;
    /// # }
    /// #
    /// # struct FindProductQueryHandler;
    /// #
    /// # #[async_trait]
    /// # impl QueryHandler<FindProductQuery> for FindProductQueryHandler {
    /// #    async fn handle(&self, query: FindProductQuery) -> Result<Product, FindProductError> {
    /// #       Ok(Product {
    /// #           id: query.product_id,
    /// #           name: "Wireless Mouse".to_string(),
    /// #           price: 29.99,
    /// #       })
    /// #   }
    /// # }
    /// use qonduit::registry::QueryHandlerRegistry;
    /// use qonduit::query::QueryBus;
    ///
    /// // Create and set up a registry with our query handlers
    /// let mut registry = QueryHandlerRegistry::new();
    ///
    /// // The registry manages associations between query types and handlers
    /// // Here we register our FindProductQuery with its handler
    /// registry.register::<FindProductQuery>(FindProductQueryHandler { /* ... */ });
    ///
    /// // The QueryBus uses the registry to find handlers for queries
    /// let query_bus = QueryBus::new(registry);
    ///
    /// // When we dispatch a query through the bus, it:
    /// // 1. Uses the query's type to find the right handler in the registry
    /// // 2. Calls the handler's handle method with the query
    /// // 3. Returns the result
    /// let result = query_bus.dispatch(FindProductQuery { product_id: 1 }).await;
    ///
    /// match result {
    ///     Ok(product) => {
    ///         # assert_eq!(product.name, "Wireless Mouse".to_string());
    ///         println!("Product name: {}", product.name);
    ///     },
    ///     Err(err) => {
    ///         # assert!(false);
    ///         match err {
    ///             FindProductError::ProductNotFound => println!("Product not found"),
    ///             FindProductError::InvalidQuery => println!("Invalid product query"),
    ///         }
    ///     }
    /// }
    /// # });
    /// ```
    pub async fn dispatch<Q: Query>(&self, query: Q) -> Result<Q::Response, Q::Error> {
        match self.registry.get_handler::<Q>() {
            Some(handler) => handler.handle(query).await,
            None => {
                panic!(
                    "No handler registered for query: {:?}",
                    std::any::type_name::<Q>()
                );
            }
        }
    }
}
