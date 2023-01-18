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
    pub async fn get_all_currencies(db: &DbConn) -> Result<Vec<currency::Model>, DbErr> {
        Currency::find().all(db).await
    }
    pub async fn find_currency_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<currency::Model>, DbErr> {
        Currency::find_by_id(id).one(db).await
    }
}
