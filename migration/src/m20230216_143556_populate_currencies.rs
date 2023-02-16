use crate::data::get_currencies_str;
use entity::currency;
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

        // //Specify relative path
        // let currencies_json_path = Path::new("migration/src/data/currencies.json");

        // // Open the file using `std::fs::File`.
        // let file = File::open(currencies_json_path).unwrap();
        // let reader = BufReader::new(file);

        // Parse the JSON string into a `Value` using `serde_json::from_reader`.
        let currencies_json: Value = serde_json::from_str(get_currencies_str()).unwrap();

        // Extract the JSON array from the `Value`.
        if let Value::Array(currencies) = currencies_json {
            // Iterate through each object in the array.
            for mut currency in currencies {
                currency["selected"] = serde_json::Value::Bool(false);
                // let mut currency_object = currency.as_object_mut().unwrap();
                // currency_object["selected"] =
                let new_currency = currency::ActiveModel::from_json(currency).unwrap();

                // new_currency.set_from_json(currency).unwrap();
                // new_currency.save(&transaction).await.unwrap();
                new_currency.insert(&transaction).await?;
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
