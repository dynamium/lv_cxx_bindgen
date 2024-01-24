use log::warn;
use rayon::prelude::*;

use crate::api_map::{APIMap, Enum};

#[derive(Debug, Clone)]
pub struct EnumerationMember {
    pub identifier: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Enumeration {
    pub identifier: Option<String>,
    pub r_type: String,
    pub members: Vec<EnumerationMember>,
}

fn get_common_string(Strings: Vec<String>) -> String {
    let mut common_string = String::new();

    let minimum_length = Strings.iter().map(|s| s.len()).min().unwrap();

    for i in 0..minimum_length {
        for string in &Strings {
            if string.chars().nth(i).unwrap() != Strings[0].chars().nth(i).unwrap() {
                return common_string;
            } else {
                common_string.push(string.chars().nth(i).unwrap());
            }
        }
    }

    common_string
}

fn get_score_match(s1 : &String, s2 : &String) -> usize {
    let mut score = 0;
    let minimum_length = s1.len().min(s2.len());

    // Insensitive to case
    let s1 = s1.to_lowercase();
    let s2 = s2.to_lowercase();

    for i in 0..minimum_length {
        if s1.chars().nth(i).unwrap() == s2.chars().nth(i).unwrap() {
            score += 1;
        }
    }
    score
}

fn get_enum_identifier(api_map: &APIMap, enumeration: &Enum) -> String {
    if enumeration.identifier != "anonymous" {
        return enumeration.identifier.clone();
    }

    let mut common_string =
        get_common_string(enumeration.members.iter().map(|m| m.0.clone()).collect());

    if common_string == "" {
        warn!(
            "Could not find common string for enum members: {:#?}",
            enumeration.members
        );
        return "anonymous".to_string();
    }

    // Looking for best match

    let mut best_match = String::new();
    let mut best_match_score = 0;
    
    for typedef in &api_map.typedefs {
        let score = get_score_match(&common_string, &typedef.identifier);
        if score > best_match_score {
            best_match = typedef.identifier.clone();
            best_match_score = score;
        }
    }

    if best_match_score > 0 {
        return best_match;
    }

    
    warn!(
        "Could not find typedef match for enum members: {:#?}",
        enumeration.members
    );
    "anonymous".to_string()
}

/// Convert a snake_case or SCREAMING_SNAKE_CASE string to PascalCase
fn convert_casing(input: &String) -> String {
    let mut result = String::new();
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

pub fn enumeration_processor(api_map: &APIMap) -> Vec<Enumeration> {
    let enumerations: Vec<Enumeration> = api_map
        .enums
        .clone()
        .par_iter()
        .map(|enumeration| Enumeration {
            identifier: Some(get_enum_identifier(api_map, enumeration)),
            r_type: enumeration.r#type.clone(),
            members: enumeration
                .members
                .clone()
                .into_iter()
                .map(|member| EnumerationMember {
                    identifier: member.0.clone(),
                    value: member.1.clone(),
                })
                .collect(),
        })
        .collect();

    enumerations
}
