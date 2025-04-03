use qonduit::query::{Query, QueryHandler};
use qonduit::{async_trait, query_bus};

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ListProductsQuery;

impl Query for ListProductsQuery {
    type Response = Vec<Product>;
    type Error = std::io::Error;
}

struct ListProductsQueryHandler;

#[async_trait]
impl QueryHandler<ListProductsQuery> for ListProductsQueryHandler {
    async fn handle(&self, _query: ListProductsQuery) -> Result<Vec<Product>, std::io::Error> {
        Ok(vec![
            Product {
                name: "Wireless Mouse".to_string(),
                price: 29.99,
            },
            Product {
                name: "Mechanical Keyboard".to_string(),
                price: 89.99,
            },
        ])
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create query bus with registered handler
    let query_bus = query_bus! {
        ListProductsQuery => ListProductsQueryHandler,
    };

    // Dispatch query and process results
    let products = query_bus.dispatch(ListProductsQuery).await?;

    // Display product information
    for product in products {
        println!("{} - ${:.2}", product.name, product.price);
    }

    Ok(())
}
