use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::types::ConvertableResult;
use crate::types::Json;
use crate::types::SettingKey;
use crate::types::Settings;
use accountant_core::settings;

pub struct SettingsMutation;

#[graphql_object(context = Context)]
impl SettingsMutation {
    async fn set(&self, context: &Context, key: SettingKey, value: Json) -> FieldResult<Settings> {
        let conn = context.get_connection();
        let key_str = Into::<String>::into(key);
        let new_settings = settings::SettingsMutation::set(conn, &key_str, value.into()).await;
        // if key == SettingKey::MainCurrency {
        //     let rate_calculator = context.get_rate_calculator();
        //     rate_calculator.calculate_rates().await;
        // }
        new_settings.convert()
    }
}
