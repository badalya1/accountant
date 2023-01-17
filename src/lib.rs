// TODO: Export rootnode of graphql api here so general journal can use it
pub use accountant_graphql_api as api;
use accountant_graphql_api::{build_schema, Schema};
pub use migration;
#[no_mangle]
pub fn get_root_node() -> Schema {
    build_schema()
}

#[no_mangle]
pub fn get_schema() -> String {
    let root = get_root_node();

    // Convert the Rust schema into the GraphQL Schema Language.
    root.as_schema_language()
}
