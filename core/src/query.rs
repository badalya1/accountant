use ::entity::{account, account::Entity as Account};
use sea_orm::*;
pub struct Query;
impl Query {
    pub async fn get_all_accounts(db: &DbConn) -> Result<Vec<account::Model>, DbErr> {
        Account::find().all(db).await
    }
}
