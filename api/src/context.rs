use accountant_core::{currency::RateCalculator, sea_orm::DatabaseConnection};
use juniper::Context as JuniperContext;

#[derive(Clone)]
pub struct Context {
    pub connection: DatabaseConnection,
    pub forex: RateCalculator,
}

impl Context {
    pub async fn new() -> Self {
        let connection = accountant_core::sea_orm::Database::connect(
            std::env::var("DATABASE_URL").expect("DATABASE_URL not provided"),
        )
        .await
        .expect("Could not connect to database");

        let forex = RateCalculator::new(connection.clone()).await;

        Context { connection, forex }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
    pub fn get_rate_calculator(&self) -> &RateCalculator {
        &self.forex
    }
}

impl JuniperContext for Context {}
