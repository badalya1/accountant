use crate::data::get_preferences_str;
use entity::preference;
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
        let preferences_json: Value = serde_json::from_str(get_preferences_str()).unwrap();

        // Extract the JSON array from the `Value`.
        if let Value::Array(preferences) = preferences_json {
            // Iterate through each object in the array.
            for preference in preferences {
                let new_preference = preference::ActiveModel::from_json(preference).unwrap();

                new_preference.insert(&transaction).await?;
            }
        }

        // Commit it
        transaction.commit().await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        Ok(())
    }
}
