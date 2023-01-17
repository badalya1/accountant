pub mod db;
pub mod graphql;
pub mod types;

use std::env;

use warp::{http::Response, Filter};

use db::Database;
pub use graphql::schema::build_schema;

pub use graphql::schema::Schema;

#[tokio::main]
pub async fn serve() {
    env::set_var("RUST_LOG", "warp_server");
    env_logger::init();

    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    log::info!("Listening on 127.0.0.1:8080");
    let database = Database::new().await;
    let state = warp::any().map(move || database.clone());
    let graphql_filter = juniper_warp::make_graphql_filter(build_schema(), state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await
}
