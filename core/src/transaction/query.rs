use ::entity::{transaction, transaction::Entity as Transaction};
use sea_orm::*;

pub struct TransactionQuery;

impl TransactionQuery {
    pub async fn get_all_transactions(db: &DbConn) -> Result<Vec<transaction::Model>, DbErr> {
        Transaction::find().all(db).await
    }
}
