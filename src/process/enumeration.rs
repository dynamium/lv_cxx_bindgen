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
    pub members: Vec<EnumerationMember>,
}

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

pub fn enumeration_processor(api_map: &APIMap) -> Vec<Enumeration> {    
    let enumerations: Vec<Enumeration> = api_map
        .enums
        .clone()
        .par_iter()
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
                    identifier: member.identifier.clone(),
                    value: member.value.clone(),
                })
                .collect(),
        })
        .collect();

    println!("enumerations: {:#?}", enumerations);
    enumerations
}
