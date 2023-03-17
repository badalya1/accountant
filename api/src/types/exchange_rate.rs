use accountant_core::currency;
use entity::exchange_rate;
use juniper::{graphql_object, FieldResult, ID};

use crate::context::Context;

use super::{ConvertableVec, Currency};

#[derive(Debug, Clone)]
pub struct CalculatedRate {
    model: currency::CalculatedRate,
}

impl From<currency::CalculatedRate> for CalculatedRate {
    fn from(value: currency::CalculatedRate) -> Self {
        CalculatedRate { model: value }
    }
}

#[graphql_object(context = Context)]
impl CalculatedRate {
    fn rate(&self) -> &f64 {
        &self.model.rate
    }
    fn path(&self) -> FieldResult<Vec<ConversionNode>> {
        todo!()
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

#[graphql_object(context = Context)]
impl ConversionNode {
    fn currency(&self) -> &f64 {
        &self.model.rate
    }
    fn rate(&self) -> FieldResult<ExchangeRate> {
        &self.model.rate
    }
    fn inverted(&self) -> &bool {
        &self.model.inverted
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
impl ExchangeRate {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
    fn from(&self, context: &Context) -> FieldResult<Currency> {
        &self.model.rate
    }
    fn to(&self, context: &Context) -> FieldResult<Currency> {
        todo!()
    }
    fn rate(&self) -> &f64 {
        &self.model.rate
    }
}
