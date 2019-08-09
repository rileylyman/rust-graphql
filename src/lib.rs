extern crate serde_json;

use serde_json::{Result, Value};

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

type Id = usize;

enum Datum {
    Int(i32),
    Str(String),
    Type(String, Id),
}

struct Node {
    fields: HashMap<String, Datum>,
}

//
// Boolean represents whether or not is array type
//
enum DataType {
    Int(bool),
    String(bool),
    NodeTypeNotParsed(String, bool),
    NodeType(NodeType, bool),
}

struct NodeType {
    type_name: String,
    fields: HashMap<String, DataType>
}

struct Schema {
    types: Vec<NodeType>
}

enum FieldQuery {
    Primitive(String),
    SubQuery(String, Query),
}

struct Query {
    fields_to_query: Vec<FieldQuery>
}

struct UserQuery {
    start: String,
    query: Query,
}

enum QueryResult {
    Success(String),
    Failure,
}

fn parse_schema(path: &Path) -> std::io::Result<Schema> {
    unimplemented!()
}

fn construct_schema(path: &str) -> std::io::Result<Schema> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let types = Vec::new();

    enum State {
        LookingForNextType,
        LookingForOpeningBrace,
        LookingForBody,
        LookingForClosingBrace,
    }

    let state = State::LookingForNextType;
    let lines = contents.split('\n');
    for line in lines {
        match state {
            State::LookingForNextType => {
                let mut name;
                let words = line.split(' ').collect::<Vec<&str>>();
                if let Some(first) = words.get(0) {
                    if *first != "type" {
                        panic!("Parse Error: Expected keyword `type`")
                    }
                } else {
                    //
                    // Skip empty line
                    //
                    continue;
                }

                if let Some(second) = words.get(1) {
                    name = second.to_string();
                } else {
                    panic!("Parse Error: Expected type name after keyword `type`")
                }

                state = State::LookingForOpeningBrace;
            }
            State::LookingForBody => {
                let name_and_type = line.split(':').collect::<Vec<&str>>();
                if name_and_type.len() != 2 {
                    return Err("j");
                }
                if let (Some(name), Some(ty)) =
                       (name_and_type.get(0), name_and_type.get(1)) 
                { 
                    
                } 
                else {
                    return Err("j");
                }
            }
        }
    }

    Ok(Schema {
        types
    })
}

fn construct_query(path: &str) -> std::io::Result<UserQuery> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.split('\n');
    for line in lines {
        
    }

    Ok(UserQuery {
        start: String::new(),
        query: Query { fields_to_query: Vec::new() }
    })
}

fn run_query(start: String, query: &Query, schema: &Schema) -> QueryResult {

    QueryResult::Failure
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
