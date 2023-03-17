use ::entity::{exchange_rate, exchange_rate::Entity as ExchangeRate};

use sea_orm::*;

pub struct ExchangeRateQuery;

impl ExchangeRateQuery {
    pub async fn get_from(db: &DbConn, from_id: i32) -> Result<Vec<exchange_rate::Model>, DbErr> {
        ExchangeRate::find()
            .filter(exchange_rate::Column::FromId.eq(from_id))
            .all(db)
            .await
    }
    pub async fn get_to(db: &DbConn, to_id: i32) -> Result<Vec<exchange_rate::Model>, DbErr> {
        ExchangeRate::find()
            .filter(exchange_rate::Column::ToId.eq(to_id))
            .all(db)
            .await
    }

    // pub async fn get_main_currency(db: &DbConn) -> Result<currency::Model, DbErr> {
    //     let main_currency_id = Preference::find_by_id()
    //     Currency::find_by_id(id).one(db).await
    // }
}
