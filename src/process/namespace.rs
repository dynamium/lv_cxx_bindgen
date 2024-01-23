use log::debug;
use rayon::prelude::*;
use std::collections::HashSet;

use crate::{
    api_map::{self, APIMap},
    conf::NamespacesConfig,
};

use super::{Argument, Function, Namespace};

pub fn namespace_generator(functions: &[Function], conf: &NamespacesConfig) -> Vec<Namespace> {
    let namespace_list: Vec<_> = functions
        .par_iter()
        .filter(|f| !f.identifier.starts_with("_"))
        .filter(|f| f.identifier.split('_').collect::<Vec<_>>().len() > 2)
        .map(|f| f.identifier.split('_').collect::<Vec<_>>()[1].to_string())
        .collect::<HashSet<_>>() // exists only to make all items unique
        .into_iter()
        .map(|n| {
            (
                n.clone(),
                functions
                    .into_iter()
                    .filter(|f| f.identifier.split('_').collect::<Vec<_>>()[1] == n)
                    .map(|f| Function {
                        identifier: f.identifier.clone().replace(&format!("lv_{n}_"), ""),
                        return_type: f.return_type.clone(),
                        args: f
                            .args
                            .clone()
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

    debug!("{:#?}", namespace_list);
    todo!()
}

/// Returns a namespace group with a provided name and functions.
///
/// # Arguments
///
/// * `name` - Name of the namespace group.
/// * `functions` - A slice of functions, from which the required functions are extracted.
pub fn make_namespace_group<'a>(
    name: &str,
    functions: &[api_map::Function],
    _blacklist: &[String],
) -> (Vec<Namespace>, Vec<Function>) {
    let _temp = functions
        .iter()
        .filter(|func| func.identifier.starts_with(&format!("lv_{}_", name)));
    // debug!("{_temp:#?}");
    todo!()
}
