use std::collections::HashMap;
use super_server::helpers::{generate_tables, get_tables};
use tera::{Context, Tera};

fn main() {
    let tables = get_tables("./src/schema.rs");
    generate_tables("./dist/db_models.rs", tables);
}
