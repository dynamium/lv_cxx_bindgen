use log::{debug, trace};
use rayon::prelude::*;
use std::collections::HashSet;

use crate::conf::{ExcludeInclude, NamespacesConfig};

use super::{Argument, Function, Namespace};

pub fn namespace_generator(
    functions: &[Function],
    conf: &NamespacesConfig,
) -> Vec<Namespace> {
    let namespace_list: Vec<_> = functions
        .par_iter()
        .filter(|f| !f.identifier.starts_with("_")) // possibly filters out privates
        // if the amount of words is less than 2 in a function name, it doesn't have a
        // namespace, therefore should be excluded from this
        .filter(|f| f.identifier.split('_').collect::<Vec<_>>().len() > 2)
        .map(|f| extract_namespace(&f.identifier, &conf.include))
        .collect::<HashSet<_>>() // exists only to make all items unique
        .into_iter()
        .map(|n| Namespace {
            identifier: n.clone(),
            members: functions
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
        })
        .collect();

    return namespace_list;
}

fn extract_namespace(func: &str, namespace_include: &[ExcludeInclude]) -> String {
    let possible_namespace = func.split('_').collect::<Vec<_>>()[1];
    trace!("Function: {func}, possible namespace: {possible_namespace}");
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
        debug!("Found namespace {found_namespace:#?} for {func}");
        return found_namespace.unwrap();
    }

    return possible_namespace.to_string();
}
