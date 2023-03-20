use super::ConvertableVec;
use accountant_core::currency;
use entity::exchange_rate;
use juniper::{graphql_object, FieldError, FieldResult, ID};
use migration::DbErr;

use crate::context::Context;

use super::Currency;

#[derive(Debug, Clone)]
pub struct CalculatedRate {
    model: currency::CalculatedRate,
}

impl From<currency::CalculatedRate> for CalculatedRate {
    fn from(value: currency::CalculatedRate) -> Self {
        CalculatedRate { model: value }
    }
}

#[derive(Debug, Clone)]
pub struct ConversionNode {
    model: currency::ConversionNode,
}

impl From<currency::ConversionNode> for ConversionNode {
    fn from(value: currency::ConversionNode) -> Self {
        ConversionNode { model: value }
    }
}

#[derive(Debug, Clone)]
pub struct ExchangeRate {
    model: exchange_rate::Model,
}

impl From<exchange_rate::Model> for ExchangeRate {
    fn from(value: exchange_rate::Model) -> Self {
        ExchangeRate { model: value }
    }
}

#[graphql_object(context = Context)]
impl ConversionNode {
    async fn currency(&self, context: &Context) -> FieldResult<Currency> {
        let db = context.get_connection();
        Ok(
            currency::CurrencyQuery::find_currency_by_id(db, self.model.currency_id)
                .await
                .expect("This must exist")
                .expect("Yes")
                .into(),
        )
    }

    fn inverted(&self) -> &bool {
        &self.model.inverted
    }
}

#[graphql_object(context = Context)]
impl CalculatedRate {
    fn rate(&self) -> &f64 {
        &self.model.rate
    }

    fn path(&self) -> Vec<ConversionNode> {
        let path = &self.model.path;
        let new_path: Vec<ConversionNode> = path.clone().convert();

        new_path.clone()
    }
}

#[graphql_object(context = Context)]
impl ExchangeRate {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
    async fn from(&self, context: &Context) -> FieldResult<Currency> {
        let db = context.get_connection();
        let currency = currency::CurrencyQuery::find_currency_by_id(&db, self.model.from_id)
            .await
            .expect("Could not query currency")
            .expect("Could not query currency");
        Ok(currency.into())
    }
    async fn to(&self, context: &Context) -> FieldResult<Currency> {
        let db = context.get_connection();
        let currency = currency::CurrencyQuery::find_currency_by_id(&db, self.model.to_id)
            .await
            .expect("Could not query currency")
            .expect("Could not query currency");

        Ok(currency.into())
    }
    fn rate(&self) -> &f64 {
        &self.model.rate
    }
}
