use qonduit::command::{Command, CommandHandler};
use qonduit::{async_trait, command_bus};
use std::error::Error;
use std::fmt;

// Product entity
#[derive(Debug)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: f64,
}

// Command error types
#[derive(Debug)]
pub enum CreateProductError {
    InvalidPrice,
    NameTooShort,
}

// Implement Display for the error
impl fmt::Display for CreateProductError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreateProductError::InvalidPrice => write!(f, "Price must be greater than zero"),
            CreateProductError::NameTooShort => write!(f, "Product name is too short"),
        }
    }
}

// Implement Error trait
impl Error for CreateProductError {}

// Command to create a new product
#[derive(Debug)]
pub struct CreateProductCommand {
    pub name: String,
    pub price: f64,
}

impl Command for CreateProductCommand {
    // Returns the ID of the created product
    type Response = u64;
    type Error = CreateProductError;
}

// Handler for product creation
pub struct CreateProductCommandHandler {
    pub next_id: u64, // In a real app, this would be in a database
}

#[async_trait]
impl CommandHandler<CreateProductCommand> for CreateProductCommandHandler {
    async fn handle(&self, cmd: CreateProductCommand) -> Result<u64, CreateProductError> {
        // Validate command data
        if cmd.price <= 0.0 {
            return Err(CreateProductError::InvalidPrice);
        }
        if cmd.name.len() < 3 {
            return Err(CreateProductError::NameTooShort);
        }

        // In a real application, we would save to a database
        let product = Product {
            id: self.next_id,
            name: cmd.name,
            price: cmd.price,
        };

        println!("Created product: {:?}", product);
        Ok(product.id)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create command bus with registered handler
    let command_bus = command_bus! {
        CreateProductCommand => CreateProductCommandHandler { next_id: 1 },
    };

    // Execute commands
    let valid_command = CreateProductCommand {
        name: "Wireless Headphones".to_string(),
        price: 59.99,
    };

    let product_id = command_bus.dispatch(valid_command).await?;
    println!("Product created with ID: {}", product_id);
    Ok(())
}
