use entity::currency;
use juniper::{graphql_object, ID};

use crate::context::Context;

#[derive(Debug, Clone)]
pub struct Currency {
    model: currency::Model,
}

impl From<currency::Model> for Currency {
    fn from(value: currency::Model) -> Self {
        Currency { model: value }
    }
}

#[graphql_object(context = Context)]
impl Currency {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
    fn name(&self) -> &str {
        &self.model.name
    }
    fn code(&self) -> &str {
        &self.model.code
    }
    fn symbol(&self) -> &Option<String> {
        &self.model.symbol
    }
    fn numeric_code(&self) -> &i32 {
        &self.model.numeric_code
    }
    fn digits(&self) -> &i32 {
        &self.model.digits
    }
    fn selected(&self) -> &bool {
        &self.model.selected
    }
    fn rate($self, context: &Context) -> FieldResult<CalculatedRate> {
        todo!()

    }
}
