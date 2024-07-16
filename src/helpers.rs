use crate::models::{Column, Table};

fn write_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    // Create the necessary directories if they don't exist
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Open the file in write mode, create if it doesn't exist
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn get_tables() -> Vec<Table>{

Table {
    name: "sometable".to_string()
}
}
// let tera = match Tera::new("templates/*.tera") {
//     Ok(t) => t,
//     Err(e) => {
//         println!("Parsing error(s): {}", e);
//         std::process::exit(1);
//     }
// };

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

// // Create a context and insert the struct name and fields
// let mut context = Context::new();
// context.insert("struct_name", &struct_name);
// context.insert("fields", &fields);

// // Render the template with the provided context
// let rendered = tera.render("db_model.tera", &context).unwrap();
// println!("{}", rendered);