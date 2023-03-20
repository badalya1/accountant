use ::entity::{preference as settings, preference::Entity as Settings};
use sea_orm::*;

pub struct SettingsQuery;

impl SettingsQuery {
    pub async fn get(db: &DbConn, key: &str) -> Result<settings::Model, DbErr> {
        let settings = Settings::find()
            .filter(settings::Column::Field.eq(key))
            .one(db)
            .await?;
        settings.ok_or(DbErr::RecordNotFound("Settings not found".to_owned()))
    }
}
