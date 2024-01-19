// The simplest templating engine in existence.
// It only replaces the found marker once, because this templating
// engine expects only a single marker with one ID to exist in
// a source file.
pub fn paste_in_template(id: &str, source: &str, string: &str) -> String {
    if source.find(id).is_some() {
        source.replace(&format!("// MARKER: {id}"), string)
    } else {
        return String::from(source);
    }
}
