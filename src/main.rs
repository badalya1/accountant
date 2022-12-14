#[macro_use]
extern crate juniper;

mod graphql;
mod models;

#[tokio::main]
async fn main() {
    println!("{:?} has been created", "David");
}
