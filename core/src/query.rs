use ::entity::{account, account::Entity as Account};
use ::entity::{currency, currency::Entity as Currency};
use sea_orm::*;

pub struct Query;
impl Query {
    pub async fn get_all_accounts(db: &DbConn) -> Result<Vec<account::Model>, DbErr> {
        Account::find().all(db).await
    }
    pub async fn find_account_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<Option<account::Model>, DbErr> {
        Account::find_by_id(id).one(db).await
    }
    pub async fn get_transactions(db: &DbConn) -> Result<Vec<account::Model>, DbErr> {
        Account::find().all(db).await
    }
}
