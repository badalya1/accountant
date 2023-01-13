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
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Account::Name).string().not_null())
                    .col(ColumnDef::new(Account::Icon).string())
                    .col(ColumnDef::new(Account::CurrencyId).integer().not_null())
                    .col(ColumnDef::new(Account::AccountType).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("Account_currencyId_fkey")
                            .from(Account::Table, Account::CurrencyId)
                            .to(Currency::Table, Currency::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Account {
    Table,
    Id,
    Name,
    Icon,
    CurrencyId,
    AccountType,
}
