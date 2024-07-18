use std::{collections::HashMap, fs::{self, File, OpenOptions}, io::{Read, Write}, path::Path};

use regex::Regex;
use tera::{Context, Tera};

use crate::models::{Column, Table};



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


pub fn generate_tables(file_path: &str, tables: Vec<Table>) -> std::io::Result<()> {
    // Create the necessary directories if they don't exist
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    
    let tera = match Tera::new("templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    
    // // Define the struct name and fields
    // let struct_name = "Users".to_string();
    // let mut fields = Vec::new();
    
    // // Populate fields as a vector of maps
    // let field_map = |name: &str, typ: &str| {
    //     let mut map = HashMap::new();
    //     map.insert("name".to_string(), name.to_string());
    //     map.insert("type".to_string(), typ.to_string());
    //     map
    // };
    
    // fields.push(field_map("username", "String"));
    // fields.push(field_map("email", "String"));
    // fields.push(field_map("password", "String"));
    // fields.push(field_map("phone_number", "String"));
    
    // Create a context and insert the struct name and fields
    let mut final_result: String = Default::default();
    for table in tables.into_iter() {        
        let struct_name = table.name[0..1].to_ascii_uppercase() + &table.name[1..table.name.len()];
        let mut context = Context::new();
        context.insert("table_name", &table.name);
        context.insert("struct_name", &struct_name);
        context.insert("fields", &table.columns);
        final_result.push_str("\n");
        final_result.push_str(tera.render("db_model.tera", &context).unwrap().as_str());
        
    }

    
    // Render the template with the provided context
    // println!("{}", rendered); 
    // Open the file in write mode, create if it doesn't exist
    let mut file = File::create(file_path).unwrap();
    file.write_all(final_result.as_bytes()).unwrap();
    Ok(())
}