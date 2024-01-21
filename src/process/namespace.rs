use crate::api_map;

use super::{Function, Namespace};

fn namespace_generator() {
    // debug!("{:#?}", namespace_list);
    // for ns in namespace_list {
    //     let test = make_namespace_group(&ns, &api_map.functions, &[]);
    //     debug!("{test:#?}");
    // }
}

/// Returns a namepspace group with a provided name and functions.
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
