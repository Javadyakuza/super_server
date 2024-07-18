use serde::Serialize;

use crate::diesel_types::get_rust_eq;


trait RustilizeCol {
    fn rustilize_column(&mut self) -> Self;
}
#[derive(Default, Debug, Clone, Serialize)]
pub struct Column {
    pub name: String, 
    pub ty: String, 
    pub nullable: bool,
}

impl RustilizeCol for Column {
    fn rustilize_column(&mut self) -> Self {
        Self {
            name: self.name.clone(),
            ty:get_rust_eq(self.ty.clone()),
            nullable: self.nullable,
        }

    }
}

#[derive(Default, Debug)]
pub struct Table {
    pub name : String,
    pub columns: Vec<Column>,
    pub primary_key: String
}


impl Table {
pub fn std_struct_name (&self)  -> String{

    let tmp_chars = self.name.chars();
    let mut tmp_name  = self.name.clone();
    for (i,c) in tmp_chars.into_iter().enumerate(){
        if c == '_'{
            tmp_name  = tmp_name.replace(format!("_{}", &self.name[i+1..i+2]).as_str(),&self.name[i+1..i+2].to_ascii_uppercase());
        }
    }
    let std_name = self.name[0..1].to_ascii_uppercase() + &tmp_name[1..tmp_name.len()];
    println!("returning this one : {}", std_name);
    std_name
    
}}
