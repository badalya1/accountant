use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::types::{ConvertableVec, SettingKey, Settings};
use accountant_core::settings;

pub struct SettingsQuery;

#[graphql_object(context = Context)]
impl SettingsQuery {
    async fn list(context: &Context) -> FieldResult<Vec<Settings>> {
        let conn = context.get_connection();
        let settings = settings::SettingsQuery::get_all(conn).await?;
        let result: Vec<Settings> = settings.convert();
        Ok(result)
    }

    async fn get(context: &Context, key: SettingKey) -> FieldResult<Settings> {
        let conn = context.get_connection();
        let key_str: String = key.into();
        let settings = settings::SettingsQuery::get(conn, &key_str).await?;
        let result: Settings = settings.into();
        Ok(result)
    }
}
