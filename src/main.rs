#![allow(unused)]
// TODO: Run the main function of api here so playground can be accessed

#[cfg(debug_assertions)]
use dotenvy::dotenv;

fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    accountant_graphql_api::serve();
}
