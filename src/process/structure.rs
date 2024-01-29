use crate::api_map::APIMap;

use super::{Structure, StructureField};

pub fn structure_processor(api_map: &APIMap) -> Vec<Structure> {
    api_map
        .structs
        .clone()
        .into_iter()
        .map(|structure| Structure {
            identifier: structure.identifier,
            fields: structure
                .fields
                .into_iter()
                .map(|field| {
                    let field = field.clone();
                    StructureField {
                        identifier: field.identifier,
                        kind: field.kind,
                        bitsize: field.bitsize,
                    }
                })
                .collect(),
        })
        .collect()
}
