use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::api_map::APIMap;

#[derive(Debug, Clone)]
pub struct EnumerationMember {
    pub identifier: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Enumeration {
    pub identifier: String,
    pub r_type: String,
    pub members: Vec<EnumerationMember>,
}

pub fn enumeration_processor(api_map: &APIMap) -> Vec<Enumeration> {
    let enumerations: Vec<Enumeration> = api_map
        .enums
        .clone()
        .par_iter()
        .map(|enumeration| Enumeration {
            identifier: enumeration.identifier.clone(),
            r_type: enumeration.r#type.clone(),
            members: enumeration
                .members
                .clone()
                .into_iter()
                .map(|member| EnumerationMember {
                    identifier: member.0,
                    value: member.1,
                })
                .collect(),
        })
        .collect();

    enumerations
}
