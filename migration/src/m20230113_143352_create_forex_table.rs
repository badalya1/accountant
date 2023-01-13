use super::m20230113_143331_create_currency_table::Currency;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExchangeRate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExchangeRate::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ExchangeRate::FromId).integer().not_null())
                    .col(ColumnDef::new(ExchangeRate::ToId).integer().not_null())
                    .col(ColumnDef::new(ExchangeRate::Rate).double().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("ExchangeRate_fromId_fkey")
                            .from(ExchangeRate::Table, ExchangeRate::FromId)
                            .to(Currency::Table, Currency::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("ExchangeRate_toId_fkey")
                            .from(ExchangeRate::Table, ExchangeRate::ToId)
                            .to(Currency::Table, Currency::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExchangeRate::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ExchangeRate {
    Table,
    Id,
    FromId,
    ToId,
    Rate,
}
