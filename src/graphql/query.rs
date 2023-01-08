use crate::models::account::{Account, AccountType};
use juniper::{graphql_object, FieldResult, ID};
pub struct Query;
#[graphql_object]
impl Query {
    fn accounts() -> FieldResult<Vec<Account>> {
        let mut accounts = Vec::<Account>::new();
        accounts.push(Account {
            id: ID::new("1"),
            currency_id: "1".to_owned(),
            kind: AccountType::Vault,
            name: "Cash".to_owned(),
        });
        accounts.push(Account {
            id: ID::new("2"),
            currency_id: "1".to_owned(),
            kind: AccountType::Vault,
            name: "Cash".to_owned(),
        });

        Ok(accounts)
    }
}
