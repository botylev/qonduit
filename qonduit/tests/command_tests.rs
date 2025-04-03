use qonduit::async_trait;
use qonduit::command::{Command, CommandBus, CommandHandler};
use qonduit::registry::CommandHandlerRegistry;
use std::fmt::Debug;

// Minimal command type
#[derive(Debug)]
struct TestCommand(u32);

// Simple error type
#[derive(Debug, PartialEq)]
struct TestError;

// Implement Command trait
impl Command for TestCommand {
    type Response = u32;
    type Error = TestError;
}

// Success handler - returns the command value multiplied by 2
struct TestCommandHandler;

#[async_trait]
impl CommandHandler<TestCommand> for TestCommandHandler {
    async fn handle(&self, command: TestCommand) -> Result<u32, TestError> {
        Ok(command.0 * 2)
    }
}

#[tokio::test]
async fn test_command_bus_successful_dispatch() {
    // Create registry and register handler
    let mut registry = CommandHandlerRegistry::new();
    registry.register::<TestCommand>(TestCommandHandler);

    // Create bus with registry
    let bus = CommandBus::new(registry);

    // Dispatch command
    let result = bus.dispatch(TestCommand(21)).await;

    // Check result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[tokio::test]
#[should_panic(expected = "No handler registered for command")]
async fn test_command_bus_missing_handler() {
    // Create registry without registering any handlers
    let registry = CommandHandlerRegistry::new();

    // Create bus with empty registry
    let bus = CommandBus::new(registry);

    // This should panic since no handler is registered
    let _ = bus.dispatch(TestCommand(0)).await;
}

// ===== Command Registry Tests =====

// Command type 1 for registry tests
#[derive(Debug)]
struct Command1(u32);

impl Command for Command1 {
    type Response = u32;
    type Error = TestError;
}

struct Command1Handler;

#[async_trait]
impl CommandHandler<Command1> for Command1Handler {
    async fn handle(&self, cmd: Command1) -> Result<u32, TestError> {
        Ok(cmd.0)
    }
}

// Command type 2 for registry tests
#[derive(Debug)]
struct Command2(String);

impl Command for Command2 {
    type Response = String;
    type Error = TestError;
}

struct Command2Handler;

#[async_trait]
impl CommandHandler<Command2> for Command2Handler {
    async fn handle(&self, cmd: Command2) -> Result<String, TestError> {
        Ok(cmd.0)
    }
}

#[tokio::test]
async fn test_command_registry_empty() {
    let registry = CommandHandlerRegistry::new();
    assert!(registry.get_handler::<Command1>().is_none());
}

#[tokio::test]
async fn test_command_registry_single_registration() {
    let mut registry = CommandHandlerRegistry::new();

    // Initially no handler is registered
    assert!(registry.get_handler::<Command1>().is_none());

    // Register a handler
    registry.register::<Command1>(Command1Handler);

    // Now a handler should be available
    let handler = registry.get_handler::<Command1>();
    assert!(handler.is_some());

    // The handler should work correctly
    let result = handler.unwrap().handle(Command1(42)).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[tokio::test]
async fn test_command_registry_multiple_registrations() {
    let mut registry = CommandHandlerRegistry::new();

    // Register multiple handlers
    registry.register::<Command1>(Command1Handler);
    registry.register::<Command2>(Command2Handler);

    // Both handlers should be available
    assert!(registry.get_handler::<Command1>().is_some());
    assert!(registry.get_handler::<Command2>().is_some());

    // Test Command1 handler
    let handler1 = registry.get_handler::<Command1>().unwrap();
    let result1 = handler1.handle(Command1(42)).await;
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), 42);

    // Test Command2 handler
    let handler2 = registry.get_handler::<Command2>().unwrap();
    let result2 = handler2.handle(Command2("test".to_string())).await;
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), "test");
}

#[tokio::test]
async fn test_command_registry_handler_overwrite() {
    let mut registry = CommandHandlerRegistry::new();

    // Define a second handler for Command1
    struct AnotherCommand1Handler;

    #[async_trait]
    impl CommandHandler<Command1> for AnotherCommand1Handler {
        async fn handle(&self, cmd: Command1) -> Result<u32, TestError> {
            Ok(cmd.0 * 2) // This one doubles the value
        }
    }

    // Register first handler
    registry.register::<Command1>(Command1Handler);

    // Test first handler
    let handler1 = registry.get_handler::<Command1>().unwrap();
    let result1 = handler1.handle(Command1(42)).await;
    assert_eq!(result1.unwrap(), 42);

    // Register second handler (overwrites first one)
    registry.register::<Command1>(AnotherCommand1Handler);

    // Test second handler
    let handler2 = registry.get_handler::<Command1>().unwrap();
    let result2 = handler2.handle(Command1(42)).await;
    assert_eq!(result2.unwrap(), 84); // Should be doubled now
}
