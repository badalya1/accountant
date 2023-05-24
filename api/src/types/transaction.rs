use crate::{context::Context, types::ConvertableResult};
use accountant_core::account;
use chrono::{DateTime, Utc};
use cuid::cuid2 as cuid;
use derive_more::From;
use entity::transaction;
use juniper::{graphql_object, FieldResult, GraphQLInputObject, GraphQLUnion, ID};

use crate::types::Account;

#[derive(Debug, Clone)]
pub struct DebitTransaction {
    model: transaction::Model,
}
#[derive(Debug, Clone)]
pub struct CreditTransaction {
    model: transaction::Model,
}

#[derive(GraphQLInputObject)]
pub struct NewTransactionInput {
    pub date: String,
    pub amount: f64,
    pub description: String,
    pub account_id: ID,
    pub category_id: ID,
}
#[derive(GraphQLInputObject)]
pub struct TimespanInput {
    pub from: f64,
    pub to: f64,
}

impl From<transaction::Model> for DebitTransaction {
    fn from(value: transaction::Model) -> Self {
        DebitTransaction { model: value }
    }
}

impl From<transaction::Model> for CreditTransaction {
    fn from(value: transaction::Model) -> Self {
        CreditTransaction { model: value }
    }
}

impl From<NewTransactionInput> for transaction::Model {
    fn from(value: NewTransactionInput) -> Self {
        transaction::Model {
            id: cuid(),
            account_id: value.account_id.to_string(),
            amount: value.amount,
            notes: Some(value.description),
            date: DateTime::parse_from_rfc3339(&value.date)
                .unwrap()
                .with_timezone(&Utc)
                .to_string(),
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        }
    }
}

#[graphql_object(context = Context)]
impl DebitTransaction {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
    fn date(&self) -> &String {
        &self.model.created_at
    }
    fn amount(&self) -> &f64 {
        &self.model.amount
    }
    fn description(&self) -> &Option<String> {
        &self.model.notes
    }
    async fn toAccount(&self, context: &Context) -> FieldResult<Account> {
        let conn = context.get_connection();
        let account =
            account::AccountQuery::find_account_by_id(conn, self.model.account_id.to_string())
                .await;

        account.convert()
    }
}

#[graphql_object(context = Context)]
impl CreditTransaction {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
    fn date(&self) -> &String {
        &self.model.created_at
    }
    fn amount(&self) -> &f64 {
        &self.model.amount
    }
    fn description(&self) -> &Option<String> {
        &self.model.notes
    }
    async fn fromAccount(&self, context: &Context) -> FieldResult<Account> {
        let conn = context.get_connection();
        let account =
            account::AccountQuery::find_account_by_id(conn, self.model.account_id.to_string())
                .await;

        account.convert()
    }
}

#[derive(From, GraphQLUnion)]
#[graphql(context = Context)]
pub enum Transaction {
    Debit(DebitTransaction),
    Credit(CreditTransaction),
}

impl From<transaction::Model> for Transaction {
    fn from(value: transaction::Model) -> Self {
        match value.amount > 0.0f64 {
            true => Transaction::Debit(DebitTransaction::from(value)),
            false => Transaction::Credit(CreditTransaction::from(value)),
        }
    }
}
