use super::m20230113_143410_create_transaction_table::Transaction;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transfer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transfer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Transfer::DebitTransactionId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transfer::CreditTransactionId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("Transfer_debitTransactionId_fkey")
                            .from(Transfer::Table, Transfer::DebitTransactionId)
                            .to(Transaction::Table, Transaction::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("Transfer_debitTransactionId_fkey")
                            .from(Transfer::Table, Transfer::CreditTransactionId)
                            .to(Transaction::Table, Transaction::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transfer::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Transfer {
    Table,
    Id,
    DebitTransactionId,
    CreditTransactionId,
}
