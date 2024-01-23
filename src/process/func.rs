use super::Argument;
use super::Function;
use crate::api_map::{self, APIMap, FuncArg};
use crate::conf::FunctionsConfig;
use log::trace;
use log::{debug, error};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;

/// Processes functions into a vector with groups of those functions, organised
/// by function namespace (lv_<namespace>_<function_name>)
pub fn function_processor(api_map: &APIMap, conf: &FunctionsConfig) -> Vec<Function> {
    debug!("Conf: {:#?}", conf);
    let function_list: Vec<_> = api_map
        .functions
        .clone()
        .par_iter()
        // check if the argument list is not just a singular void arg
        .filter(|f| {
            f.args
                != vec![FuncArg {
                    identifier: None,
                    kind: "void".to_string(),
                }]
        })
        // remove all functions from the excludes
        .filter(|f| {
            trace!("Checking function {}", f.identifier);
            for exclude in &conf.exclude {
                let re = Regex::new(&exclude).unwrap();
                if re.is_match(&f.identifier) {
                    debug!("{} matched to {exclude}", f.identifier);
                    return false;
                } else {
                    continue;
                }
            }
            return true;
        })
        // convert arguments and return types to more idimoatic C++ types
        .map(|f| Function {
            identifier: f.identifier.clone(),
            return_type: f.return_type.clone(),
            args: f
                .args
                .clone()
                .into_iter()
                .map(|arg| Argument {
                    identifier: arg.identifier,
                    kind: match arg.kind.as_str() {
                        "char*" => "std::string".to_string(),
                        _ => arg.kind,
                    },
                })
                .collect(),
        })
        .collect();

    function_list
}
