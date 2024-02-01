use super::{Enumeration, EnumerationMember};
use log::debug;

use crate::api_map::APIMap;

/// Convert a snake_case or SCREAMING_SNAKE_CASE string to PascalCase
fn convert_to_pascal_case(input: &str) -> String {
    let mut result = String::new();
    result.reserve(input.len()); // Pre-allocate memory to avoid re-allocations
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push_str(&c.to_uppercase().to_string());
            capitalize_next = false;
        } else {
            match c.to_lowercase().next() {
                Some(c) => result.push(c),
                None => result.push(c),
            }
        }
    }

    result
}

/// Remove the common string from the beginning of the identifier
pub fn remove_common_string(input: &str, identifier: &str) -> String {
    let mut input = input.to_lowercase();

    if input.starts_with("_") {
        input.remove(0);
    }

    let mut identifier = identifier.to_lowercase();

    if identifier.starts_with("_") {
        identifier.remove(0);
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
                Some(convert_to_pascal_case(&enumeration.identifier.clone().unwrap()))
            } else {
                debug!("Enumeration identifier is None");
                None
            },
            members: enumeration
                .members
                .clone()
                .into_iter()
                .map(|member| EnumerationMember {
                    identifier: convert_to_pascal_case(&if enumeration.identifier.is_some() {
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

    enumerations
}
