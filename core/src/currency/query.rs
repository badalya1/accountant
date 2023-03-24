use crate::settings::SettingsQuery;
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

    pub async fn get_main_currency_id(db: &DbConn) -> Result<i32, DbErr> {
        let main_currency_id_setting = SettingsQuery::get(db, "MAIN_CURRENCY").await?;
        let main_currency_id = main_currency_id_setting.value.parse::<i32>();
        match main_currency_id {
            Ok(id) => Ok(id),
            Err(_) => Err(DbErr::RecordNotFound("MAIN_CURRENCY is empty".to_string())),
        }
    }
}
