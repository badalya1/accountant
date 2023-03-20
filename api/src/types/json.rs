// Scalar juniper type for json

use juniper::GraphQLScalarValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, GraphQLScalarValue, Serialize, Deserialize)]
#[graphql(transparent)]
pub struct Json(pub String);

impl Into<Value> for Json {
    fn into(self) -> Value {
        serde_json::from_str(&self.0).unwrap()
    }
}
