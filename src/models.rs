
pub struct Column {
    pub name: String, 
    pub ty: String, 
    pub nullable: bool,
}

pub struct Table {
    pub name : String,
    pub fields: Vec<Column>,
    pub primary_key: String
}

