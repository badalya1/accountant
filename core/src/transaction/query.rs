use ::entity::{transaction, transaction::Entity as Transaction};
use sea_orm::{prelude::ChronoDateTimeUtc, *};

pub struct TransactionQuery;

impl TransactionQuery {
    pub async fn get_all_transactions(
        db: &DbConn,
        from: ChronoDateTimeUtc,
        to: ChronoDateTimeUtc,
    ) -> Result<Vec<transaction::Model>, DbErr> {
        Transaction::find()
            .filter(transaction::Column::Date.between(from, to))
            .all(db)
            .await
    }
}
