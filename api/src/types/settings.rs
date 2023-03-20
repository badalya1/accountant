use accountant_core::settings;

use juniper::{graphql_object, FieldResult, GraphQLInputObject, ID};

use crate::{context::Context, types::ConvertableVec};

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
    fn name(&self) -> &str {
        &self.model.name
    }
}
