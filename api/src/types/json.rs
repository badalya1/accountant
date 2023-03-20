// Scalar juniper type for json

use juniper::GraphQLScalarValue;

#[derive(Clone, Debug, GraphQLScalarValue)]
#[graphql(transparent)]
pub struct Json(pub String);
