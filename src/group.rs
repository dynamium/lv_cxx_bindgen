use log::debug;

use crate::{
    conf::ConfigGenClassItem,
    parse::{Function, TypedValue},
};

#[derive(Debug, Clone)]
pub struct Namespace {
    pub identifier: String,
    pub members: Vec<Function>,
}

pub struct Class {
    pub identifier: String,
    pub constructor_args: Vec<TypedValue>,
    pub members: Vec<Function>,
}

pub fn group_in_namespaces<'a>(
    names: &[String],
    functions: &'a [Function],
) -> (Vec<Namespace>, Vec<&'a Function>) {
    let mut used_functions = vec![];
    let mut namespaces = vec![];
    for name in names {
        debug!("Going through {} namespace", name);
        let function_start = format!("lv_{}_", name);
        let temp = functions
            .iter()
            .filter(|func| func.identifier.starts_with(&function_start));

        used_functions.append(&mut temp.clone().collect::<Vec<_>>());
        let functions_for_namespace: Vec<_> = temp
            .map(|func| Function {
                return_type: func.return_type.clone(),
                identifier: func.identifier.replace(&function_start, ""),
                args: func.args.clone(),
                original_ident: Some(func.identifier.clone()),
            })
            .collect();
        debug!(
            "Made this list of functions: {:#?}",
            functions_for_namespace
        );
        let name_temp = name.clone();
        namespaces.push(Namespace {
            identifier: name_temp,
            members: functions_for_namespace,
        })
    }

    (namespaces, used_functions)
}

pub fn group_in_classes<'a>(
    _names: &[ConfigGenClassItem],
    _functions: &'a [Function],
) -> (Vec<Class>, &'a [Function]) {
    todo!();
}
