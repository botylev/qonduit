use qonduit::async_trait;
use qonduit::query::{Query, QueryBus, QueryHandler};
use qonduit::registry::QueryHandlerRegistry;
use std::fmt::Debug;

// Error type for tests
#[derive(Debug, PartialEq)]
struct TestError;

// ===== Query Bus Tests =====

// Minimal query type for bus tests
#[derive(Debug)]
struct TestQuery(u32);

// Minimal response type for bus tests
#[derive(Debug, PartialEq, Clone)]
struct TestResponse(u32);

// Implement Query trait
impl Query for TestQuery {
    type Response = TestResponse;
    type Error = TestError;
}

// Handler implementation - returns query value multiplied by 2
struct TestQueryHandler;

#[async_trait]
impl QueryHandler<TestQuery> for TestQueryHandler {
    async fn handle(&self, query: TestQuery) -> Result<TestResponse, TestError> {
        Ok(TestResponse(query.0 * 2))
    }
}

#[tokio::test]
async fn test_query_bus_successful_dispatch() {
    // Create registry and register handler
    let mut registry = QueryHandlerRegistry::new();
    registry.register::<TestQuery>(TestQueryHandler);

    // Create bus with registry
    let bus = QueryBus::new(registry);

    // Dispatch query
    let result = bus.dispatch(TestQuery(21)).await;

    // Check result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TestResponse(42));
}

#[tokio::test]
#[should_panic(expected = "No handler registered for query")]
async fn test_query_bus_missing_handler() {
    // Create registry without registering any handlers
    let registry = QueryHandlerRegistry::new();

    // Create bus with empty registry
    let bus = QueryBus::new(registry);

    // This should panic since no handler is registered
    let _ = bus.dispatch(TestQuery(0)).await;
}

// ===== Query Registry Tests =====

// Query type 1 for registry tests
#[derive(Debug)]
struct Query1(u32);

#[derive(Debug, PartialEq)]
struct Response1(u32);

impl Query for Query1 {
    type Response = Response1;
    type Error = TestError;
}

struct Query1Handler;

#[async_trait]
impl QueryHandler<Query1> for Query1Handler {
    async fn handle(&self, query: Query1) -> Result<Response1, TestError> {
        Ok(Response1(query.0))
    }
}

// Query type 2 for registry tests
#[derive(Debug)]
struct Query2(String);

#[derive(Debug, PartialEq)]
struct Response2(String);

impl Query for Query2 {
    type Response = Response2;
    type Error = TestError;
}

struct Query2Handler;

#[async_trait]
impl QueryHandler<Query2> for Query2Handler {
    async fn handle(&self, query: Query2) -> Result<Response2, TestError> {
        Ok(Response2(query.0))
    }
}

#[tokio::test]
async fn test_query_registry_empty() {
    let registry = QueryHandlerRegistry::new();
    assert!(registry.get_handler::<Query1>().is_none());
}

#[tokio::test]
async fn test_query_registry_single_registration() {
    let mut registry = QueryHandlerRegistry::new();

    // Initially no handler is registered
    assert!(registry.get_handler::<Query1>().is_none());

    // Register a handler
    registry.register::<Query1>(Query1Handler);

    // Now a handler should be available
    let handler = registry.get_handler::<Query1>();
    assert!(handler.is_some());

    // The handler should work correctly
    let result = handler.unwrap().handle(Query1(42)).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Response1(42));
}

#[tokio::test]
async fn test_query_registry_multiple_registrations() {
    let mut registry = QueryHandlerRegistry::new();

    // Register multiple handlers
    registry.register::<Query1>(Query1Handler);
    registry.register::<Query2>(Query2Handler);

    // Both handlers should be available
    assert!(registry.get_handler::<Query1>().is_some());
    assert!(registry.get_handler::<Query2>().is_some());

    // Test Query1 handler
    let handler1 = registry.get_handler::<Query1>().unwrap();
    let result1 = handler1.handle(Query1(42)).await;
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), Response1(42));

    // Test Query2 handler
    let handler2 = registry.get_handler::<Query2>().unwrap();
    let result2 = handler2.handle(Query2("test".to_string())).await;
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), Response2("test".to_string()));
}

#[tokio::test]
async fn test_query_registry_handler_overwrite() {
    let mut registry = QueryHandlerRegistry::new();

    // Define a second handler for Query1
    struct AnotherQuery1Handler;

    #[async_trait]
    impl QueryHandler<Query1> for AnotherQuery1Handler {
        async fn handle(&self, query: Query1) -> Result<Response1, TestError> {
            Ok(Response1(query.0 * 2)) // This one doubles the value
        }
    }

    // Register first handler
    registry.register::<Query1>(Query1Handler);

    // Test first handler
    let handler1 = registry.get_handler::<Query1>().unwrap();
    let result1 = handler1.handle(Query1(42)).await;
    assert_eq!(result1.unwrap(), Response1(42));

    // Register second handler (overwrites first one)
    registry.register::<Query1>(AnotherQuery1Handler);

    // Test second handler
    let handler2 = registry.get_handler::<Query1>().unwrap();
    let result2 = handler2.handle(Query1(42)).await;
    assert_eq!(result2.unwrap(), Response1(84)); // Should be doubled now
}
