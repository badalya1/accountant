use ::entity::preference;
use sea_orm::*;
use serde_json::Value;

pub struct SettingsMutation;

impl SettingsMutation {
    pub async fn set(db: &DbConn, key: &str, value: Value) -> Result<preference::Model, DbErr> {
        //Set the value of already existing setting given by key.
        //If the setting does not exist, create it.
        let setting_lookup = preference::Entity::find()
            .filter(preference::Column::Field.eq(key))
            .one(db)
            .await
            .unwrap_or(None);
        if setting_lookup.is_none() {
            preference::ActiveModel {
                field: Set(key.to_owned()),
                value: Set(value.to_string()),
                ..Default::default()
            }
            .insert(db)
            .await
        } else {
            let mut updated_setting = setting_lookup.unwrap().into_active_model();
            updated_setting.value = Set(value.to_string());
            updated_setting.update(db).await
        }
    }
    pub async fn update(
        _db: &DbConn,
        _category_id: i32,
        _category: preference::Model,
    ) -> Result<preference::Model, DbErr> {
        todo!();
    }
}
