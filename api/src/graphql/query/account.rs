use crate::db::Database;
use crate::types::Account;
use accountant_core::Query;
use async_graphql::{Context, Object, Result};
use entity::async_graphql;

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {}
