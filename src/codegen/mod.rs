pub mod ast;

use self::ast::NamespaceDeclaration;
use crate::conf::CxxVersion;
use ast::{FunctionCall, FunctionDeclaration, Node};
use log::error;

impl Node for FunctionDeclaration<'_> {
    fn gen_source(&self, target: &CxxVersion) -> String {
        let mut buf = String::new();
        let mut header = String::new();
        header.push_str(&format!("{} {}", self.return_type, self.identifier));
        header.push_str(&make_comma_list(self.args, true, |arg| {
            if let Some(ident) = &arg.identifier {
                return Some(format!("{} {}", arg.kind, ident));
            }
            error!("Function argument doesn't have an identifier, that's a problem'");
            None
        }));
        buf.push_str(&make_code_block(&header, || {
            let mut buf = String::new();
            for node in self.body {
                buf.push_str(&node.gen_source(target));
            }
            buf
        }));

        buf
    }
}

impl Node for FunctionCall<'_> {
    fn gen_source(&self, _target: &CxxVersion) -> String {
        let mut buf = String::new();
        for component in self.path {
            buf.push_str(&format!("{component}::"));
        }
        buf.push_str(self.identifier);
        buf.push_str(&make_comma_list(self.args, true, |arg| {
            Some(arg.to_string())
        }));

        buf
    }
}

impl Node for NamespaceDeclaration<'_> {
    fn gen_source(&self, target: &CxxVersion) -> String {
        let mut buf = String::new();
        buf.push_str(&make_code_block(
            &format!("namespace {}", self.identifier),
            || {
                let mut buf = String::new();

                for func in &self.members {
                    buf.push_str(&func.gen_source(target));
                }

                buf
            },
        ));

        buf
    }
}

fn make_code_block<F: Fn() -> String>(header: &str, content: F) -> String {
    let mut buf = format!("{header} {{\n");
    buf.push_str(&content());
    buf.push('}');
    buf.push('\n');
    buf
}

fn make_comma_list<T, F: Fn(&T) -> Option<String>>(
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
