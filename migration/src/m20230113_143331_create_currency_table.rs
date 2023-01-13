use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Currency::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Currency::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Currency::Name).string().not_null())
                    .col(ColumnDef::new(Currency::Code).string().not_null())
                    .col(ColumnDef::new(Currency::Symbol).string())
                    .col(ColumnDef::new(Currency::NumericCode).integer().not_null())
                    .col(
                        ColumnDef::new(Currency::Digits)
                            .integer()
                            .not_null()
                            .default(2),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Currency::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Currency {
    Table,
    Id,
    Name,
    Code,
    Symbol,
    NumericCode,
    Digits,
}
