extern crate serde_json;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use serde_json::{Value};
use regex::Regex;

use std::collections::HashMap;
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
#[derive(Debug)]
enum DataType {
    Int(bool),
    String(bool),
    NodeTypeNotParsed(String, bool),
    NodeType(usize, bool),
}

#[derive(Debug)]
struct NodeType {
    type_name: String,
    fields: HashMap<String, DataType>
}

#[derive(Debug)]
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

enum ParseResult {
    Ok()
}

fn proper_type_name(name: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("[A-Za-z]+[0-9]?").unwrap();
    }
    RE.is_match(name)
}

fn construct_schema(path: &str) -> std::io::Result<Schema> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut types = Vec::new();

    enum State {
        LookingForNextType,
        LookingForOpeningBrace,
        LookingForBodyOrClosingBrace,
    }

    let mut state = State::LookingForNextType;
    let lines = contents.split('\n');
    let mut type_name = String::new();
    let mut fields = HashMap::new();
    for line in lines {
        match state {
            State::LookingForNextType => {
                let words = line.split_whitespace().collect::<Vec<&str>>();
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
                    if proper_type_name(second) {
                        type_name = second.to_string();
                    }
                    else {
                        panic!(format!("Invalid type name {}", second));
                    }
                } else {
                    panic!("Parse Error: Expected type name after keyword `type`")
                }

                if let Some(third) = words.get(2) {
                    if *third == "{" {
                        state = State::LookingForBodyOrClosingBrace;
                    }
                    else {
                        panic!(format!("Parse Error: Unexpected Token: {}", third))
                    }
                } else {
                    state = State::LookingForOpeningBrace;
                }

            }
            State::LookingForOpeningBrace => {
                let tokens = line.split_whitespace().collect::<Vec<&str>>();
                if tokens.len() == 0 {
                    //
                    // Skip empty line
                    //
                    continue;
                }
                if tokens.len() != 1 {
                    panic!(format!("Expecting an opening brace on a new line, got {:?}", tokens));
                }
                if let Some(maybe_brace) = tokens.get(0) {
                    if *maybe_brace == "{" {
                        state = State::LookingForBodyOrClosingBrace;
                        continue;
                    }
                    else {
                        panic!(format!("Unexpected token: {}", maybe_brace));
                    }
                } else {
                    unreachable!()
                }
            }
            State::LookingForBodyOrClosingBrace => {
                let mut maybe_bracket = line.split_whitespace().collect::<Vec<&str>>();
                if maybe_bracket.len() == 1 {
                    if *maybe_bracket.get(0).unwrap() == "}" {
                        //
                        // push to types and clean up
                        //
                        types.push(NodeType {
                            type_name: type_name,
                            fields: fields,
                        });
                        type_name = String::new();
                        fields = HashMap::new();
                        state = State::LookingForNextType;
                        continue;
                    } else {
                        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
                    }
                }

                let mut name_and_type = line.split(':').collect::<Vec<&str>>();
                for s in name_and_type.iter_mut() {
                    *s = s.trim();
                }
                if name_and_type.len() != 2 {
                    return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
                }
                if let (Some(name), Some(ty)) =
                       (name_and_type.get(0), name_and_type.get(1)) 
                { 
                    let data_type = if *ty == "Int" {
                        DataType::Int(false)
                    } else if *ty == "String" {
                        DataType::String(false)
                    } else if *ty == "[Int]" {
                        DataType::Int(true)
                    } else if *ty == "[String]" {
                        DataType::String(true)
                    } else {
                        let mut arr_type = false;
                        if &ty[0..1] == "[" && &ty[(ty.len() - 1)..] == "]" {
                            arr_type = true;
                        }
                        DataType::NodeTypeNotParsed(
                            if arr_type {
                                ty[1..ty.len() - 1].to_string() 
                            } else {
                                ty.to_string()
                            }, 
                            arr_type
                        )
                    };
                    fields.insert(name.to_string(), data_type);
                } 
                else {
                    return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
                }
            }
        }
    }

    let mut name_to_idx = HashMap::new();
    for (i, node_type) in types.iter().enumerate() {
        name_to_idx.insert(node_type.type_name.clone(), i);
    }

    for node_type in types.iter_mut() {
        for data_type in node_type.fields.values_mut() {
            match data_type {
                DataType::NodeTypeNotParsed(name, arr) => {
                    *data_type = DataType::NodeType(*name_to_idx.get(name).unwrap(), *arr);
                }
                _ => {}
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
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn schema() {
        let schema = construct_schema("./src/schema.ql").unwrap();
        println!("Schema: {:?}", schema);
    }
}
