use accountant_core::category;
use entity::category::*;

use juniper::{graphql_object, FieldResult, GraphQLInputObject, ID};

use crate::{context::Context, types::ConvertableVec};

#[derive(Debug, Clone)]
pub struct Category {
    model: Model,
}

#[derive(GraphQLInputObject)]
pub struct UpdateCategoryInput {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
}

impl From<Model> for Category {
    fn from(value: Model) -> Self {
        Category { model: value }
    }
}

#[graphql_object(context = Context)]
impl Category {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
    fn name(&self) -> &str {
        &self.model.name
    }
    fn icon(&self) -> &Option<String> {
        &self.model.icon
    }
    async fn children(&self, context: &Context) -> FieldResult<Vec<Category>> {
        let conn = context.get_connection();

        let categories = category::CategoryQuery::get_children(conn, self.model.id).await?;

        Ok(categories.convert())
    }
}
