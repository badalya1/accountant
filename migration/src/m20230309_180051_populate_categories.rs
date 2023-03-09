use crate::data::get_categories_str;
use entity::category;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, TransactionTrait},
};

use serde_json::Value;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Get the connection and start a transaction
        let db = manager.get_connection();
        let transaction = db.begin().await?;

        // Parse the JSON string into a `Value` using `serde_json::from_reader`.
        let categories_json: Value = serde_json::from_str(get_categories_str()).unwrap();

        // Extract the JSON array from the `Value`.
        if let Value::Array(categories) = categories_json {
            // Iterate through each object in the array.
            for category in categories {
                let new_category = category::ActiveModel::from_json(category).unwrap();

                new_category.insert(&transaction).await?;
            }
        }

        // Commit it
        transaction.commit().await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
