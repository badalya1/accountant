use ::entity::{account, account::Entity as Account};
use ::entity::{transaction, transaction::Entity as Transaction};
use sea_orm::*;

pub struct AccountQuery;

impl AccountQuery {
    pub async fn get_all_accounts(db: &DbConn) -> Result<Vec<account::Model>, DbErr> {
        Account::find().all(db).await
    }
    pub async fn find_account_by_id(db: &DbConn, id: String) -> Result<account::Model, DbErr> {
        let account = Account::find_by_id(id)
            .one(db)
            .await
            .expect("Could not execute query on account");

        account.ok_or(DbErr::RecordNotFound("Account not found".to_owned()))
    }
    pub async fn get_account_balance(db: &DbConn, id: String) -> Result<f64, DbErr> {
        let transactions = Transaction::find()
            .filter(transaction::Column::AccountId.eq(id))
            .all(db)
            .await?;

        let balance = transactions.iter().fold(0.0, |acc, t| acc + t.amount);

        Ok(balance)
    }
}
