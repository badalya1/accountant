use super::currency::*;
use crate::db::Database;
use cuid::cuid2 as cuid;
use entity::account;
use juniper::{graphql_object, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject, ID};
use std::str::FromStr;
use strum_macros::{EnumProperty, EnumString};
#[derive(GraphQLEnum, EnumString, Debug)]
pub enum AccountType {
    Vault,
    Credit,
    Loan,
    Promise,
}

#[derive(GraphQLInputObject)]
pub struct NewAccountInput {
    pub name: String,
    pub currency_id: i32,
    pub account_type: AccountType,
    pub icon: Option<String>,
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

impl From<NewAccountInput> for account::Model {
    fn from(value: NewAccountInput) -> Self {
        account::Model {
            id: cuid(),
            name: value.name,
            currency_id: value.currency_id,
            account_type: value.account_type.into(),
            icon: value.icon,
        }
    }
}

impl Into<String> for AccountType {
    fn into(self) -> String {
        format!("{:?}", self)
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

    async fn currency(&self, context: &Database) -> FieldResult<Currency> {
        let conn = context.get_connection();
        let currency: Currency =
            accountant_core::Query::find_currency_by_id(conn, self.model.currency_id)
                .await
                .map_err(|e| e.to_string())
                .unwrap()
                .expect("Could not find the currency associated with this account")
                .into();
        Ok(currency)
    }
}

// impl ConvertableVec<account::Model, Account> for Vec<account::Model> {}
