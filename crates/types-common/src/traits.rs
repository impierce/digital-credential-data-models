use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, TimeZone, Utc};
use macro_derive::GenPaths;

pub enum Multiplicity {
    One,
    Many,
}

pub struct SchemaData {
    pub src_schema: String,
    pub json_path: String,
    pub tgt_schema: String,
    pub multiplicity: Multiplicity,
}

pub trait AddSchemaTypes {
    fn add_schema_types(map: &mut HashMap<String, Vec<SchemaData>>);
}

#[derive(GenPaths)]
pub struct Address {
    pub street: String,
    pub number: usize,
}

#[derive(GenPaths)]
pub struct Person {
    name: String,
    sur_name: String,
    age: usize,
    address: Address,
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
            fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
        })*
    }
}

impl_T!(for usize, u8, u16, u32, u64, u128);
impl_T!(for isize, i8, i16, i32, i64, i128);
impl_T!(for String, bool, PathBuf);

//impl<T: TimeZone> AddSchemaTypes for DateTime<T> {
    //fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
//}


//impl AddSchemaTypes for String {
    //fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
//}

//impl AddSchemaTypes for usize {
    //fn add_schema_types(_map: &mut HashMap<String, Vec<SchemaData>>) {}
//}

