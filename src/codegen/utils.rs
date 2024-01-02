pub fn make_code_block<F: Fn() -> String>(header: &str, content: F) -> String {
    let mut buf = format!("{header} {{\n");
    buf.push_str(&content());
    buf.push('}');
    buf.push('\n');
    buf
}

pub fn make_comma_list<T, F: Fn(&T) -> Option<String>>(
    list: &[T],
    use_braces: bool,
    callback: F,
) -> String {
    let mut buf = String::new();
    if use_braces {
        buf.push('(');
    }
    for (i, item) in list.iter().enumerate() {
        if let Some(item) = callback(item) {
            buf.push_str(&item);
        }
        if i != list.len() - 1 {
            buf.push(',');
        }
    }
    if use_braces {
        buf.push(')');
    }

    buf
}
