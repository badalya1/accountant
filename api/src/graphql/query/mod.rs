use crate::{
    db::Database,
    types::{Account, ConvertableVec},
};
use accountant_core;
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    async fn get_accounts(&self, context: &Database) -> FieldResult<Vec<Account>> {
        let conn = context.get_connection();
        let accounts = accountant_core::Query::get_all_accounts(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap();
        let result: Vec<Account> = accounts.convert();
        Ok(result)
    }
}
