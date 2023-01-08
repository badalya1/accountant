use crate::models::money::Currency;
use juniper::{graphql_object, FieldResult, GraphQLEnum, ID};

#[derive(GraphQLEnum)]
pub enum AccountType {
    Vault,
    Credit,
    Loan,
    Promise,
}

pub struct Account {
    pub id: ID,
    pub kind: AccountType,
    pub name: String,
    pub currency_id: String,
}

#[graphql_object]
impl Account {
    fn id(&self) -> &ID {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn kind(&self) -> &AccountType {
        &self.kind
    }
    fn currency(&self) -> FieldResult<Currency> {
        Ok(Currency {
            id: ID::new("2"),
            code: "CAD".to_owned(),
            name: "Canadian Dollar".to_owned(),
            digits: 2,
            numeric_code: 940,
            symbol: Some("$".to_owned()),
        }
        .to_owned())
    }
}
