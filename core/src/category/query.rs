use ::entity::{category, category::Entity as Category};
use sea_orm::*;

pub struct CategoryQuery;

impl CategoryQuery {
    pub async fn list_roots(db: &DbConn) -> Result<Vec<category::Model>, DbErr> {
        Category::find()
            .filter(category::Column::ParentId.is_null())
            .all(db)
            .await
    }
    pub async fn get(db: &DbConn, id: i32) -> Result<category::Model, DbErr> {
        let category = Category::find_by_id(id)
            .one(db)
            .await
            .expect("Could not execute query on category");

        category.ok_or(DbErr::RecordNotFound("Category not found".to_owned()))
    }

    pub async fn get_children(
        db: &DatabaseConnection,
        category_id: i32,
    ) -> Result<Vec<category::Model>, DbErr> {
        let category = Category::find_by_id(category_id)
            .one(db)
            .await?
            .expect("Could not find category");
        let children = Category::find()
            .filter(category::Column::ParentId.eq(category.id))
            .all(db)
            .await?;
        Ok(children)
    }
}
