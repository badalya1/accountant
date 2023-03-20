use juniper::{graphql_object, FieldResult, ID};

use crate::context::Context;
use crate::types::{Category, UpdateCategoryInput};

pub struct CategoryMutation;

#[graphql_object(context = Context)]
impl CategoryMutation {
    async fn update(
        &self,
        _context: &Context,
        _category_id: ID,
        _value: UpdateCategoryInput,
    ) -> FieldResult<Category> {
        // let conn = context.get_connection();
        // let id: IDi32 = category_id.into();
        // let old_category = category::CategoryQuery::get(conn, id.0)
        //     .await
        //     .expect("Could not find the category");
        // let updated_category =
        //     category::CategoryMutation::update(conn, id.0.into(), value).await;
        // updated_category.convert()

        todo!();
    }
}
