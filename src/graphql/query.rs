use crate::models::account::{Account, AccountType};
use crate::models::money::Currency;
use juniper::{graphql_object, FieldResult};
pub struct Query;

#[graphql_object]
impl Query {
    pub fn hello(&self) -> FieldResult<&str> {
        Ok("hello world")
    }
    fn accounts() -> FieldResult<Vec<Account>> {
        let mut accounts = Vec::<Account>::new();
        accounts.push(Account {
            currency: Currency {
                code: "USD".to_owned(),
                name: "US Dollar".to_owned(),
                digits: 2,
                numeric_code: 940,
                symbol: Some("$".to_owned()),
            },
            kind: AccountType::Vault,
            name: "Cash".to_owned(),
        });

        Ok(accounts)
    }
}
