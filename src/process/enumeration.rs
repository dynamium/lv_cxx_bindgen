use super::{Enumeration, EnumerationMember};
use log::debug;

use crate::api_map::{APIMap, EnumMember};

/// Convert a snake_case or SCREAMING_SNAKE_CASE string to PascalCase. Only for ASCII
/// strings, no UTF-8.
fn convert_to_pascal_case(input: &str) -> String {
    let mut result = String::new();
    result.reserve(input.len()); // Pre-allocate memory to avoid re-allocations
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
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

    if input.starts_with('_') {
        input.remove(0);
    }

    let mut identifier = identifier.to_lowercase();

    if identifier.starts_with('_') {
        identifier.remove(0);
    }

    let mut input_iter = input.chars().peekable();
    let mut identifier_iter = identifier.chars();

    while input_iter.peek() == identifier_iter.next().as_ref() {
        input_iter.next();
    }

    input_iter.collect()
}

/// Find the largest common string in a vector of strings
pub fn find_common_string(strings: Vec<&str>) -> String {
    // If the input is empty, return an empty string
    if strings.len() < 2 {
        return String::new();
    }

    // Shortest string
    let min_length = strings.iter().map(|s| s.len()).min().unwrap();

    let mut prefix = String::new();

    'outer: for i in 0..min_length {
        let char_at_index = strings[0].chars().nth(i).unwrap();
        // Check if the character at the current index is the same in all strings
        for string in &strings {
            // If the character at the current index is not the same in all strings, break all loops
            if string.chars().nth(i).unwrap() != char_at_index {
                break 'outer;
            }
        }
        prefix.push(char_at_index);
    }

    prefix
}

pub fn enumeration_processor(api_map: &APIMap) -> Vec<Enumeration> {
    let enumerations: Vec<Enumeration> = api_map
        .enums
        .clone()
        .into_iter()
        .map(|mut enumeration| {
            // Nuke all "internal" members that start with an underscore
            enumeration
                .members
                .retain(|member| !member.identifier.starts_with('_'));

            let members_identifiers: Vec<&str> = enumeration
                .members
                .iter()
                .map(|member| member.identifier.as_str())
                .collect();

            let common_identifier = find_common_string(members_identifiers.clone());

            let members: Vec<EnumerationMember> = enumeration.members
                .clone()
                .into_iter()
                .map(|member| {
                    let identifier = convert_to_pascal_case(&remove_common_string(
                        &member.identifier,
                        &common_identifier,
                    ));
                    EnumerationMember {
                        identifier,
                        value: member.value,
                    }
                })
                .collect();

            Enumeration {
                identifier: enumeration.identifier.clone(),
                members,
            }
        })
        .collect();

    enumerations
}
