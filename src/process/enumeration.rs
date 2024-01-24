

use crate::api_map::APIMap;

#[derive(Debug, Clone)]
pub struct EnumerationMember {
    pub identifier: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Enumeration {
    pub identifier: Option<String>,
    pub members: Vec<EnumerationMember>,
}

pub fn enumeration_processor(api_map: &APIMap) -> Vec<Enumeration>
{
    let mut enumerations: Vec<Enumeration> = vec![];

    enumerations
}