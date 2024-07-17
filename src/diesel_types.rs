
pub fn types(sql_type: String) -> String {

  match sql_type.as_str() {
    "Nullable<T>" => { "Option<T>".to_string()},
    "SmallInt" => { "i16".to_string()},
    "Integer" => { "i32".to_string()},
    "BigInt" => { "i64".to_string()},
    "Numeric" => { "bigdecimal::BigDecimal".to_string()},
    "Float" => { "f32".to_string()},
    "Double" => { "f64".to_string()},
    "SmallInt" => { "i16".to_string()},
    "Integer" => { "i32".to_string()},
    "BigInt" => { "i64".to_string()},
    "Money" => { "Cents".to_string()},
    "Text" => { "String, &str".to_string()},
    "Text" => { "String, &str".to_string()},
    "Text" => { "String, &str".to_string()},
    "Binary" => { "Vec<u8>, &u8".to_string()},
    "Timestamp" => { "chrono::NaiveDateTime".to_string()},
    "Timestamptz" => { "chrono::DateTime".to_string()},
    "Date" => { "chrono::NaiveDate".to_string()},
    "Time" => { "chrono::NaiveTime".to_string()},
    "Interval" => { "PgInterval".to_string()},
    "Bool" => { "bool".to_string()},
    "Cidr" => { "ipnetwork::IpNetwork".to_string()},
    "Inet" => { "ipnetwork::IpNetwork".to_string()},
    "MacAddr" => { "[u8; 6]".to_string()},
    "(user-defined)" => { "String, enum".to_string()},
    "Uuid" => { "uuid::Uuid".to_string()},
    "Json" => { "serde_json::Value".to_string()},
    "Jsonb" => { "serde_json::Value".to_string()},
    "Array<T>" => { "Vec<T>, Vec<Option<T>>, &[T], &[[Option<T>]]".to_string()},
    "Range<Integer>" => { "Bound<i32>".to_string()},
    "Range<BigInt>" => { "Bound<i64>".to_string()},
    "Range<Numeric>" => { "Bound<bigdecimal::BigDecimal>".to_string()},
    "Range<Timestamp>" => { "Bound<chrono::NaiveDateTime>".to_string()},
    "Range<Timestamptz>" => { "Bound<chrono::DateTime>".to_string()},
    "Range<Date>" => { "Bound<chrono::NaiveDate>".to_string()}
    _ => {
      panic!("wrong type provided !")
    }
  }

}
