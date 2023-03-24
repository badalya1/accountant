use ::entity::currency;
use ::entity::currency::Entity as Currency;
use sea_orm::*;

use crate::settings::SettingsMutation;

pub struct CurrencyMutation;

impl CurrencyMutation {
    pub async fn create_currency(
        db: &DbConn,
        currency: currency::Model,
    ) -> Result<currency::Model, DbErr> {
        let new_currency = currency::ActiveModel::from(currency);
        new_currency.insert(db).await
    }
    pub async fn set_currency_selected(
        db: &DbConn,
        currency_id: i32,
        value: bool,
    ) -> Result<currency::Model, DbErr> {
        let currency = Currency::find_by_id(currency_id).one(db).await;

        let mut currency: currency::ActiveModel = currency
            .unwrap()
            .ok_or("Could not find currency")
            .unwrap()
            .into();
        currency.selected = Set(value);
        currency.update(db).await
    }

    pub async fn set_currency_main(
        db: &DbConn,
        currency_id: i32,
    ) -> Result<currency::Model, DbErr> {
        let currency = Currency::find_by_id(currency_id).one(db).await;

        let currency: _ = currency.unwrap().ok_or("Could not find currency").unwrap();
        SettingsMutation::set(db, "MAIN_CURRENCY", currency.id.into()).await?;

        Ok(currency)
    }
}
