use log::debug;
use rayon::prelude::*;
use std::collections::HashSet;

use crate::{
    api_map::{self, APIMap},
    conf::{ExcludeInclude, NamespacesConfig},
};

use super::{Argument, Function, Namespace};

pub fn namespace_generator(functions: &[Function], conf: &NamespacesConfig) -> Vec<Namespace> {
    let namespace_list: Vec<_> = functions
        .par_iter()
        .filter(|f| !f.identifier.starts_with("_"))
        .filter(|f| f.identifier.split('_').collect::<Vec<_>>().len() > 2)
        .map(|f| extract_namespace(&f.identifier, &conf.include))
        .collect::<HashSet<_>>() // exists only to make all items unique
        .into_iter()
        .map(|n| {
            (
                n.clone(),
                functions
                    .into_iter()
                    .filter(|f| extract_namespace(&f.identifier, &conf.include).as_str() == n)
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

fn extract_namespace(func: &str, namespace_include: &[ExcludeInclude]) -> String {
    let possible_namespace = func.split('_').collect::<Vec<_>>()[1];
    let mut found_namespace = None;
    if namespace_include
        .into_iter()
        .find(|i| {
            found_namespace = i
                .namespaces
                .clone()
                .into_iter()
                .find(|ns| ns.starts_with(possible_namespace));
            return found_namespace.is_some();
        })
        .is_some()
    {
        return found_namespace.unwrap();
    }

    return possible_namespace.to_string();
}
