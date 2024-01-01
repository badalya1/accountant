use ::entity::{preference as settings, preference::Entity as Settings};
use sea_orm::*;

pub struct SettingsQuery;

impl SettingsQuery {
    pub async fn get_all(db: &DbConn) -> Result<Vec<settings::Model>, DbErr> {
        let settings = Settings::find().all(db).await?;
        Ok(settings)
    }
    pub async fn get(db: &DbConn, key: &str) -> Result<settings::Model, DbErr> {
        println!("Getting settings for {}", key);
        let settings = Settings::find()
            .filter(settings::Column::Field.eq(key))
            .one(db)
            .await?;
        settings.ok_or(DbErr::RecordNotFound("Setting not found".to_owned()))
    }
}
