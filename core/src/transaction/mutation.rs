use ::entity::transaction;
use sea_orm::*;

pub struct TransactionMutation;

impl TransactionMutation {
    pub async fn create_transaction(
        db: &DbConn,
        account: transaction::Model,
    ) -> Result<transaction::Model, DbErr> {
        let new_transaction = transaction::ActiveModel::from(account);
        new_transaction.insert(db).await
    }
}
