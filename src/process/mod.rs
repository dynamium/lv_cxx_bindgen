use std::collections::HashSet;

use log::debug;

use crate::api_map::APIMap;

use self::func::make_namespace_group;

mod func;

pub fn make_api_map(api_map: APIMap) {
    let namespace_list: HashSet<String> = (&api_map
        .functions)
        .into_iter()
        .map(|f| f.identifier.split('_').collect::<Vec<_>>()[1].to_string())
        .collect();
    for ns in namespace_list {
        let test = make_namespace_group(&ns, &api_map.functions, &[]);
        debug!("{test:#?}");
    }
}
