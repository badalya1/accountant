use accountant_core::sea_orm::DatabaseConnection;
use juniper::Context as JuniperContext;

#[derive(Clone)]
pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Self {
        let connection = accountant_core::sea_orm::Database::connect(
            std::env::var("DATABASE_URL").expect("DATABASE_URL not provided"),
        )
        .await
        .expect("Could not connect to database");

        Database { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

impl JuniperContext for Database {}
