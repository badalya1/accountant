use cuid::cuid2 as cuid;
use entity::transaction;
use juniper::{graphql_interface, graphql_object, GraphQLEnum, GraphQLInputObject, ID};

#[graphql_interface]
pub trait Transaction {
    fn id(&self) -> ID;
    fn date(&self) -> &String;
    fn amount(&self) -> &f64;
    fn description(&self) -> Option<&String>;
}

#[derive(Debug, Clone)]
pub struct Income {
    model: transaction::Model,
}

#[derive(GraphQLEnum)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(GraphQLInputObject)]
pub struct NewTransactionInput {
    pub r#type: TransactionType,
    pub date: String,
    pub amount: f64,
    pub description: String,
    pub account_id: ID,
    pub category_id: ID,
}

impl From<transaction::Model> for Income {
    fn from(value: transaction::Model) -> Self {
        Income { model: value }
    }
}

impl From<NewTransactionInput> for transaction::Model {
    fn from(value: NewTransactionInput) -> Self {
        transaction::Model {
            id: cuid(),
            account_id: value.account_id.to_string(),
            amount: value.amount,
            created_at: "0".to_owned(),
            updated_at: "0".to_owned(),
            notes: Some(value.description),
        }
    }
}

#[graphql_object(impl = TransactionValue)]
impl Transaction for Income {
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
}
