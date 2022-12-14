mod query;

use juniper::{EmptyMutation, EmptySubscription, RootNode};
pub use query::Query;

pub fn get_schema() -> String {
    let root = RootNode::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );

    // Convert the Rust schema into the GraphQL Schema Language.
    root.as_schema_language()
}
