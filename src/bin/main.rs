use std::collections::HashMap;
use super_server::helpers::get_tables;
use tera::{Context, Tera};

fn main() {
    get_tables("./src/schema.rs");
}
