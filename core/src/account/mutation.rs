use ::entity::account;
use sea_orm::*;

pub struct AccountMutation;

impl AccountMutation {
    pub async fn create_account(
        db: &DbConn,
        account: account::Model,
    ) -> Result<account::Model, DbErr> {
        let new_account = account::ActiveModel::from(account);
        new_account.insert(db).await
    }
}
