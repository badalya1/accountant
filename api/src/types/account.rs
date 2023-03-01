use super::{currency::*, IDi32};
use crate::db::Database;
use accountant_core::currency;
use cuid::cuid2 as cuid;
use entity::account;
use juniper::{graphql_object, FieldResult, GraphQLEnum, GraphQLInputObject, ID};
use std::str::FromStr;
use strum_macros::EnumString;

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
    pub currency_id: ID,
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
        let currency_id: IDi32 = value.currency_id.into();
        account::Model {
            id: cuid(),
            name: value.name,
            currency_id: currency_id.0,
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
            currency::CurrencyQuery::find_currency_by_id(conn, self.model.currency_id)
                .await
                .map_err(|e| e.to_string())
                .unwrap()
                .expect("Could not find the currency associated with this account")
                .into();
        Ok(currency)
    }
}

// impl ConvertableVec<account::Model, Account> for Vec<account::Model> {}
