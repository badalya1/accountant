use ::entity::{category, category::Entity as Category};
use sea_orm::*;

pub struct CategoryQuery;

impl CategoryQuery {
    pub async fn list(db: &DbConn) -> Result<Vec<category::Model>, DbErr> {
        Category::find().all(db).await
    }
    pub async fn get(db: &DbConn, id: i32) -> Result<category::Model, DbErr> {
        let category = Category::find_by_id(id)
            .one(db)
            .await
            .expect("Could not execute query on category");

        category.ok_or(DbErr::RecordNotFound("Category not found".to_owned()))
    }
}
