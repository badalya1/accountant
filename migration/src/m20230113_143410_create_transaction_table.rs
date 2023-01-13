use super::m20230113_143340_create_account_table::Account;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transaction::AccountId).string().not_null())
                    .col(ColumnDef::new(Transaction::Amount).double().not_null())
                    .col(
                        ColumnDef::new(Transaction::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Transaction::UpdatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Transaction::Notes).text())
                    .foreign_key(
                        ForeignKey::create()
                            .name("Transaction_accountId_fkey")
                            .from(Transaction::Table, Transaction::AccountId)
                            .to(Account::Table, Account::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Transaction {
    Table,
    Id,
    AccountId,
    Amount,
    CreatedAt,
    UpdatedAt,
    Notes,
}
