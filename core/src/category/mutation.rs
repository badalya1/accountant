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
        db: &DbConn,
        category_id: i32,
        category: category::Model,
    ) -> Result<category::Model, DbErr> {
        // let new_category = category::ActiveModel::from(category) ;
        let updated = category::Entity::find_by_id(category_id)
            .one(db)
            .await?
            .unwrap();
        return Ok(updated);
    }
}
