use entity::category;
use juniper::{graphql_object, GraphQLInputObject, ID};

use crate::db::Database;

#[derive(Debug, Clone)]
pub struct Category {
    model: category::Model,
}

#[derive(GraphQLInputObject)]
pub struct UpdateCategoryInput {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
}

impl From<category::Model> for Category {
    fn from(value: category::Model) -> Self {
        Category { model: value }
    }
}

#[graphql_object(context = Database)]
impl Category {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
    fn name(&self) -> &str {
        &self.model.name
    }
}
