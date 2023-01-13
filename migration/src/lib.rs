pub use sea_orm_migration::prelude::*;

mod m20230113_143230_init;
mod m20230113_143331_create_currency_table;
mod m20230113_143340_create_account_table;
mod m20230113_143352_create_forex_table;
mod m20230113_143410_create_transaction_table;
mod m20230113_143431_create_transfer_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230113_143230_init::Migration),
            Box::new(m20230113_143331_create_currency_table::Migration),
            Box::new(m20230113_143340_create_account_table::Migration),
            Box::new(m20230113_143352_create_forex_table::Migration),
            Box::new(m20230113_143410_create_transaction_table::Migration),
            Box::new(m20230113_143431_create_transfer_table::Migration),
        ]
    }
}
