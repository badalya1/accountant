use entity::preference::*;

use juniper::{graphql_object, GraphQLInputObject, ID};

use crate::{context::Context, types::Json};

#[derive(Debug, Clone)]
pub struct Settings {
    model: Model,
}

#[derive(GraphQLInputObject)]
pub struct UpdateSettingsInput {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
}

impl From<Model> for Settings {
    fn from(value: Model) -> Self {
        Settings { model: value }
    }
}

#[graphql_object(context = Context)]
impl Settings {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
    fn key(&self) -> &str {
        &self.model.field
    }
    fn value(&self) -> Json {
        Json(self.model.value.clone()).to_owned()
    }
}
