extern crate accountant;

use accountant::graphql::get_schema;

fn main() {
    println!("{}", get_schema());
}
