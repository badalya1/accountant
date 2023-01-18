use entity::currency;
use juniper::{graphql_object, ID};

use crate::db::Database;

#[derive(Debug, Clone)]
pub struct Currency {
    model: currency::Model,
}

impl From<currency::Model> for Currency {
    fn from(value: currency::Model) -> Self {
        Currency { model: value }
    }
}

#[graphql_object(context = Database)]
impl Currency {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
    fn name(&self) -> &str {
        &self.model.name
    }
    fn code(&self) -> &str {
        &self.model.code
    }
    fn symbol(&self) -> &Option<String> {
        &self.model.symbol
    }
    fn numeric_code(&self) -> &i32 {
        &self.model.numeric_code
    }
    fn digits(&self) -> &i32 {
        &self.model.digits
    }
}

// impl ConvertableVec<account::Model, Account> for Vec<account::Model> {}
