use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::types::ConvertableResult;
use crate::types::Settings;
use accountant_core::settings;

pub struct SettingsMutation;

#[graphql_object(context = Context)]
impl SettingsMutation {
    //set
    async fn set(&self, context: &Context, key: String, value: String) -> FieldResult<Settings> {
        let conn = context.get_connection();
        let new_settings =
            settings::SettingsMutation::set(conn, &key.to_owned(), value.into()).await;
        new_settings.convert()
    }
}
