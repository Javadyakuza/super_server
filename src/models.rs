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
pub fn std_struct_name (&mut self)  -> String{
    let tmp_name = self.name.chars();
    for (i,c) in tmp_name.into_iter().enumerate(){
        if c == '_'{
            let _ = self.name.replace(format!("_{}", &self.name[i..i+1]).as_str(),&self.name[i..i+1].to_ascii_uppercase());
        }
    }
    self.name[0..1].to_ascii_uppercase() + &self.name[1..self.name.len()]
    
}}
