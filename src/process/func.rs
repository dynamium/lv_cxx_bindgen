use super::Argument;
use super::Function;
use crate::api_map::{self, APIMap, FuncArg};
use log::{debug, error};
use std::collections::HashSet;

/// Processes functions into a vector with groups of those functions, organised
/// by function namespace (lv_<namespace>_<function_name>)
pub fn function_processor(api_map: &APIMap) -> Vec<(String, Vec<Function>)> {
    let function_list: Vec<_> = api_map
        .functions
        .clone()
        .into_iter()
        .filter(|f| {
            // check if the argument list is not just a singular void arg
            f.args
                != vec![FuncArg {
                    identifier: None,
                    kind: "void".to_string(),
                }]
        })
        .collect();

    let namespace_list: Vec<_> = (&function_list)
        .into_iter()
        .filter(|f| !f.identifier.starts_with("_"))
        .filter(|f| f.identifier.split('_').collect::<Vec<_>>().len() > 2)
        .map(|f| f.identifier.split('_').collect::<Vec<_>>()[1].to_string())
        .collect::<HashSet<_>>() // exists only to make all items unique
        .into_iter()
        .map(|n| {
            (
                n.clone(),
                function_list
                    .clone()
                    .into_iter()
                    .filter(|f| f.identifier.split('_').collect::<Vec<_>>()[1] == n)
                    .map(|f| Function {
                        identifier: f.identifier.clone().replace(&format!("lv_{n}_"), ""),
                        return_type: f.return_type,
                        args: f
                            .args
                            .into_iter()
                            .map(|arg| Argument {
                                identifier: arg.identifier,
                                kind: arg.kind,
                            })
                            .collect(),
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    debug!("{namespace_list:#?}");

    todo!()
}
