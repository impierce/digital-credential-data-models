use std::{fmt, path::PathBuf};

use chrono::{DateTime, Utc};

#[derive(PartialEq, Eq)]
pub enum Multiplicity {
    One,
    Many,
    OneOrMany,
}

impl fmt::Display for Multiplicity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => f.write_str("1"),
            Self::Many => f.write_str("*"),
            Self::OneOrMany => f.write_str("1|*"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct SchemaData {
    pub src_schema: String,
    pub src_field: String,
    pub tgt_schema: String,
    pub multiplicity: Multiplicity,
    pub optional: bool,
    // TODO add optionality var
}

impl fmt::Display for SchemaData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}, {}, {}, {}, {}",
            self.src_schema, self.src_field, self.tgt_schema, self.multiplicity, self.optional
        ))
    }
}

pub trait AddSchemaTypes {
    fn add_schema_types(data: &mut Vec<SchemaData>, parent_src_schema: &str, parent_json_path: &str, optional: bool) {}

    fn add_schema() -> bool {
        true
    }
}

pub trait SchemaList {
    fn contains_schema(&self, src_schema: &str) -> bool;
}

impl SchemaList for Vec<SchemaData> {
    fn contains_schema(&self, src_schema: &str) -> bool {
        self.iter().any(|data| &data.src_schema == src_schema)
    }
}


//impl AddSchemaTypes for Option<bool> {
//fn add_schema_types(map: &mut HashMap<String, Vec<SchemaData>>) {

//if map.contains_key("Options") {
//let data = map.get_mut("Options").unwrap();

//if data.iter().all(|d| &d.tgt_schema != "bool") {
//data.push(SchemaData {
//..
//})

//}
//}
//}
//}

//impl AddSchemaTypes for Person {
//fn add_schema_types(map: &mut HashMap<String, Vec<SchemaData>>) {
//// Type has already been added
//if map.contains_key("Person") {
//return;
//}

//let mut data = vec![];

//data.push(SchemaData {
//src_schema: "Person".to_string(),
//json_path: "$.name".to_string(),
//multiplicity: Multiplicity::One,
//tgt_schema: "String".to_string(),
//});

//data.push(SchemaData {
//src_schema: "Person".to_string(),
//json_path: "$.sur_name".to_string(),
//multiplicity: Multiplicity::One,
//tgt_schema: "String".to_string(),
//});

//data.push(SchemaData {
//src_schema: "Person".to_string(),
//json_path: "$.age".to_string(),
//multiplicity: Multiplicity::One,
//tgt_schema: "usize".to_string(),
//});

//data.push(SchemaData {
//src_schema: "Person".to_string(),
//json_path: "$.address".to_string(),
//multiplicity: Multiplicity::One,
//tgt_schema: "Address".to_string(),
//});

//map.insert("Person".to_string(), data);

//// If generc
//String::add_schema_types(map);
//}
//}

// Implement empty traits impl for all types
macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(impl AddSchemaTypes for $t {
            fn add_schema_types(_data: &mut Vec<SchemaData>, _src_schema: &str,
                _src_field: &str, _optional: bool) {}
        })*
    }
}

impl_T!(for usize, u8, u16, u32, u64, u128);
impl_T!(for isize, i8, i16, i32, i64, i128);
impl_T!(for f32, f64);
impl_T!(for String, bool, PathBuf);
impl_T!(for DateTime<Utc>);
