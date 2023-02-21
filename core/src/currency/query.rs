use ::entity::{currency, currency::Entity as Currency};
use sea_orm::*;

pub struct CurrencyQuery;

impl CurrencyQuery {
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