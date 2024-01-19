use log::error;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub identifier: String,
    pub members: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub identifier: String,
    pub constructor_args: Vec<Argument>,
    pub members: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub identifier: String,
    pub return_type: String,
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub identifier: String,
    pub r#type: String,
}

/// Returns a namepspace group with a provided name and functions.
///
/// # Arguments
///
/// * `name` - Name of the namespace group.
/// * `functions` - A slice of functions, from which the required functions are extracted.
pub fn make_namespace_group<'a>(
    name: &str,
    functions: &'a [Function],
    _blacklist: &[String],
) -> (Vec<Namespace>, Vec<&'a Function>) {
    let function_start = format!("lv_{}_", name);
    let _temp = functions
        .iter()
        .filter(|func| func.identifier.starts_with(&function_start));
    todo!()
}

/// Returns a class group with a provided name and functions.
///
/// # Arguments
///
/// * `name` - Name of the class group.
/// * `functions` - A slice of functions, from which the required functions are extracted.
/// * `blacklist` - If a function is found in the function list, but present in the blacklist, it
/// does not get included to the class.
pub fn make_class_group<'a>(
    _name: &str,
    _functions: &'a [Function],
    _blacklist: &[String],
) -> (Vec<Namespace>, Vec<&'a Function>) {
    error!("Grouping in classes is not implemented yet!");
    (vec![], vec![])
}

// pub fn group_in_namespaces<'a>(
//     names: &[String],
//     functions: &'a [Function],
// ) -> (Vec<Namespace>, Vec<&'a Function>) {
//     let mut used_functions = vec![];
//     let mut namespaces = vec![];
//     for name in names {
//         debug!("Going through {} namespace", name);
//         let function_start = format!("lv_{}_", name);
//
//         used_functions.append(&mut temp.clone().collect::<Vec<_>>());
//         let functions_for_namespace: Vec<_> = temp
//             .map(|func| Function {
//                 return_type: func.return_type.clone(),
//                 identifier: func.identifier.replace(&function_start, ""),
//                 args: func.args.clone(),
//                 original_ident: Some(func.identifier.clone()),
//             })
//             .collect();
//         debug!(
//             "Made this list of functions: {:#?}",
//             functions_for_namespace
//         );
//         let name_temp = name.clone();
//         namespaces.push(Namespace {
//             identifier: name_temp,
//             members: functions_for_namespace,
//         });
//     }
//
//     (namespaces, used_functions)
// }
