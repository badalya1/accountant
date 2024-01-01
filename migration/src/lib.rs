pub use sea_orm_migration::prelude::*;

mod data;

mod m20230113_143331_create_currency_table;
mod m20230113_143340_create_account_table;
mod m20230113_143352_create_forex_table;
mod m20230113_143410_create_transaction_table;
mod m20230113_143431_create_transfer_table;
mod m20230216_143556_populate_currencies;
mod m20230302_005050_user_preferences;
mod m20230309_180039_transaction_categories;
mod m20230309_180051_populate_categories;
mod m20231031_003705_populate_user_preferences;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230113_143331_create_currency_table::Migration),
            Box::new(m20230113_143340_create_account_table::Migration),
            Box::new(m20230113_143352_create_forex_table::Migration),
            Box::new(m20230113_143410_create_transaction_table::Migration),
            Box::new(m20230113_143431_create_transfer_table::Migration),
            Box::new(m20230216_143556_populate_currencies::Migration),
            Box::new(m20230302_005050_user_preferences::Migration),
            Box::new(m20230309_180039_transaction_categories::Migration),
            Box::new(m20230309_180051_populate_categories::Migration),
            Box::new(m20231031_003705_populate_user_preferences::Migration),
        ]
    }
}
