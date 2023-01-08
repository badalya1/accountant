use juniper::{graphql_object, FieldResult};

pub struct Subscription;

#[graphql_object]
impl Subscription {
    pub fn _empty(&self) -> FieldResult<&str> {
        Ok("ok")
    }
}
