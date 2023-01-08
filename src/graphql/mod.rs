mod mutation;
mod query;
mod subscription;

use juniper::RootNode;
pub use mutation::Mutation;
pub use query::Query;
pub use subscription::Subscription;

#[no_mangle]
pub fn get_root_node() -> RootNode<'static, Query, Mutation, Subscription> {
    RootNode::new(Query, Mutation, Subscription)
}
#[no_mangle]
pub fn get_schema() -> String {
    let root = get_root_node();

    // Convert the Rust schema into the GraphQL Schema Language.
    root.as_schema_language()
}
