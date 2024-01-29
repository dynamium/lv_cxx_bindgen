use log::warn;
use rayon::prelude::*;
use super::{Enumeration, EnumerationMember};

use crate::api_map::{APIMap, Enum};

/// Convert a snake_case or SCREAMING_SNALKE_CASE string to PascalCase
fn convert_casing(input: &String) -> String {
    let mut result = String::new();
    result.reserve(input.len());
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push_str(&c.to_uppercase().to_string());
            capitalize_next = false;
        } else {
            result.push(c); // ? : Maybe this should be c.to_lowercase()
        }
    }

    result
}

pub fn remove_common_string(input: &String, identifier: &String) -> String {
    println!("{} : {}", input, identifier);
    let mut input = input.to_lowercase();
    
    if input.starts_with("_") {
        input = input.replacen("_", "", 1);
    }

    let mut identifier = identifier.to_lowercase();

    if identifier.starts_with("_") {
        identifier = identifier.replacen("_", "", 1);
    }

    let mut input_iter = input.chars().peekable();
    let mut identifier_iter = identifier.chars();
 
    while input_iter.peek() == identifier_iter.next().as_ref() {
        input_iter.next();

    }

    input_iter.collect()
}

pub fn enumeration_processor(api_map: &APIMap) -> Vec<Enumeration> {
    let enumerations: Vec<Enumeration> = api_map
        .enums
        .clone()
        .into_iter()
        .map(|enumeration| Enumeration {
            identifier: if enumeration.identifier.is_some() {
                Some(convert_casing(&enumeration.identifier.clone().unwrap()))
            } else {
                warn!("Enumeration identifier is None");
                None
            },
            members: enumeration
                .members
                .clone()
                .into_iter()
                .map(|member| EnumerationMember {
                    identifier: convert_casing(&if enumeration.identifier.is_some() {
                        remove_common_string(
                            &member.identifier,
                            &enumeration.identifier.clone().unwrap(),
                        )
                    } else {
                        member.identifier
                    }),
                    value: member.value.clone(),
                })
                .collect(),
        })
        .collect();

    println!("enumerations: {:#?}", enumerations);
    enumerations
}
