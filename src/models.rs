
trait RustilizeCol {
    fn rustilize_column(self) -> Self;
}
#[derive(Default, Debug)]
pub struct Column {
    pub name: String, 
    pub ty: String, 
    pub nullable: bool,
}

impl RustilizeCol for Column {
    fn rustilize_column(self) -> Self {
        
    }
}

#[derive(Default, Debug)]
pub struct Table {
    pub name : String,
    pub columns: Vec<Column>,
    pub primary_key: String
}

