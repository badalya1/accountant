pub fn get_currencies_str() -> &'static str {
    //TODO: Parse the contents and only include bytes that are parsed
    return include_str!("currencies.json");
}

pub fn get_categories_str() -> &'static str {
    //TODO: Parse the contents and only include bytes that are parsed
    return include_str!("categories.json");
}
