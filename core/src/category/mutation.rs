use ::entity::category;
use sea_orm::*;

pub struct CategoryMutation;

impl CategoryMutation {
    pub async fn create_category(
        db: &DbConn,
        category: category::Model,
    ) -> Result<category::Model, DbErr> {
        let new_category = category::ActiveModel::from(category);
        new_category.insert(db).await
    }
    pub async fn update(
        _db: &DbConn,
        _category_id: i32,
        _category: category::Model,
    ) -> Result<category::Model, DbErr> {
        todo!();
    }
}
