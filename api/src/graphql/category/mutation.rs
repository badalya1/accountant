use juniper::{graphql_object, FieldResult, ID};

use crate::context::Context;
use crate::types::{Category, UpdateCategoryInput};
use crate::types::{ConvertableResult, IDi32};
use accountant_core::category;

pub struct CategoryMutation;

#[graphql_object(context = Context)]
impl CategoryMutation {
    async fn update(
        &self,
        context: &Context,
        category_id: ID,
        value: UpdateCategoryInput,
    ) -> FieldResult<Category> {
        let conn = context.get_connection();
        let id: IDi32 = category_id.into();
        let old_category = category::CategoryQuery::get(conn, id.0)
            .await
            .expect("Could not find the category");
        let updated_category =
            category::CategoryMutation::update(conn, id.0.into(), old_category).await;
        updated_category.convert()
    }
}
