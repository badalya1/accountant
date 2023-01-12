use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object]
impl Mutation {
    pub fn _empty(&self) -> FieldResult<&str> {
        Ok("ok")
    }
}
