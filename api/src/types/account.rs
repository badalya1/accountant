use std::str::FromStr;

use entity::account;
use juniper::{graphql_object, GraphQLEnum, ID};
use strum_macros::EnumString;

use crate::db::Database;

#[derive(GraphQLEnum, EnumString)]
pub enum AccountType {
    Vault,
    Credit,
    Loan,
    Promise,
}

#[derive(Debug, Clone)]
pub struct Account {
    model: account::Model,
}

impl From<account::Model> for Account {
    fn from(value: account::Model) -> Self {
        Account { model: value }
    }
}

#[graphql_object(context = Database)]
impl Account {
    fn id(&self) -> ID {
        ID::new(&(self.model.id)).clone()
    }
    fn name(&self) -> &str {
        &self.model.name
    }
    fn kind(&self) -> AccountType {
        AccountType::from_str(&self.model.account_type).unwrap()
    }
}
