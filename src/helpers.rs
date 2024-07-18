use std::{collections::HashMap, fs::{self, File, OpenOptions}, io::{Read, Write}, path::Path};

use regex::Regex;

use crate::models::{Column, Table};

fn generate_tables(file_path: &str, content: &str) -> std::io::Result<()> {
    // Create the necessary directories if they don't exist
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Open the file in write mode, create if it doesn't exist
    let mut file = File::create(file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    Ok(())
}

pub fn get_tables(schema_path: &str) -> Vec<Table>{

    // reading the json file
    let mut oo = OpenOptions::new();
    let schema_open_option = oo.write(true).read(true);
    let mut schema_file =  schema_open_option.open(Path::new(schema_path)).unwrap();
    let mut schema_content: String = Default::default();
    schema_file.read_to_string(&mut schema_content).unwrap();
    

    // pasing the tables into the tables and columns
    let no_lb = schema_content.replace("\n", "").replace("diesel::table!", "diesel::table!tablebegin");
    let mut splited: Vec<&str> = no_lb.trim().split("diesel::table!").collect();
    splited.remove(0);
    let mut tables : Vec<Table> = Default::default();
    // println!("{:?}", splited);
    for (_, element) in splited.into_iter().enumerate() {
        // println!("element {:?}", element);
        tables.push(parse_table_schema(element))
    }
    tables
}


fn parse_table_schema(schema_str: &str) -> Table {
// preparing the table named the primary key    
let idx = [schema_str.find("{").unwrap(),schema_str.find("(").unwrap(), schema_str.find(")").unwrap(), schema_str.find(") {").unwrap(),schema_str.find("}").unwrap()];
let table_name: String = schema_str[idx[0] + 1..idx[1]].trim().to_string();
let primary_key: String = schema_str[idx[1] + 1..idx[2]].trim().to_string();

// preparing the columns 
let cols_splited: Vec<&str> = schema_str[idx[3]..idx[4]].split_whitespace().collect();
let mut columns : Vec<Column> = Default::default();
for (ix, el) in cols_splited.clone().into_iter().enumerate() {
if el == "->" {
    let mut ty: String = cols_splited[ix + 1].into();
    let mut nullable: bool = false;

    if cols_splited[ix + 1].contains("Nullable") {
        nullable = true;
        ty = cols_splited[ix + 1].replace("Nullable<", "").replace(">", "");
    }
    columns.push(
    Column{
        name: cols_splited[ix - 1].into(),
        ty,
        nullable
    }
)
}
}
    let table = Table {
        name : table_name,
        primary_key,
        columns,
    };

    println!("{:?} \n ", table);
    table
}
