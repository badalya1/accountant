#![allow(unused)]

extern crate juniper;
mod db;
mod graphql;
mod models;

fn main() {
    // println!("{}", graphql::get_schema());
    db::main();
}
