pub mod ast;
mod utils;

use self::{
    ast::{Comment, NamespaceDeclaration, VariableDeclaration},
    utils::{make_code_block, make_comma_list},
};
use crate::conf::CxxVersion;
use ast::{FunctionCall, FunctionDeclaration, Node};
use log::error;

impl<N: Node> Node for VariableDeclaration<'_, N> {
    fn gen_source(&self, target: &CxxVersion) -> String {
        let mut buf = String::new();
        if self.is_static {
            buf.push_str("static ");
        }
        buf.push_str(&format!("{} {}", self.kind, self.identifier));
        if let Some(assignment) = &self.assignment {
            buf.push_str(" = ");
            buf.push_str(&assignment.gen_source(target));
        }

        buf.push_str(";\n");
        buf
    }
}

impl Node for FunctionDeclaration {
    fn gen_source(&self, target: &CxxVersion) -> String {
        let mut buf = String::new();
        let mut header = String::new();
        header.push_str(&format!("{} {}", self.return_type, self.identifier));
        header.push_str(&make_comma_list(&self.args, true, |arg| {
            if let Some(ident) = &arg.identifier {
                return Some(format!("{} {}", arg.kind, ident));
            }
            error!("Function argument doesn't have an identifier, that's a problem'");
            None
        }));
        buf.push_str(&make_code_block(&header, || {
            let mut buf = String::new();
            for node in &self.body {
                buf.push_str(&node.gen_source(target));
            }
            buf
        }));

        return buf;
    }
}

impl Node for FunctionCall {
    fn gen_source(&self, _target: &CxxVersion) -> String {
        let mut buf = String::new();
        for component in &self.path {
            buf.push_str(&format!("{component}::"));
        }
        buf.push_str(&self.identifier);
        buf.push_str(&make_comma_list(&self.args, true, |arg| {
            Some(arg.to_string())
        }));

        buf
    }
}

impl Node for NamespaceDeclaration {
    fn gen_source(&self, target: &CxxVersion) -> String {
        let mut buf = String::new();
        buf.push_str(&make_code_block(
            &format!("namespace {}", self.identifier),
            || {
                let mut buf = String::new();
                for member in &self.members {
                    buf.push_str(&member.gen_source(target));
                }
                buf
            },
        ));

        buf
    }
}

impl Node for Comment<'_> {
    fn gen_source(&self, _target: &CxxVersion) -> String {
        let mut buf = String::new();
        if self.multiline {
            buf.push_str("/**\n");
            for line in self.content.split("\n") {
                buf.push_str(&format!("* {line}\n"));
            }
            buf.push_str("*/");
        } else {
            buf.push_str("// ");
            buf.push_str(&self.content.replace("\n", " "));
        }
        return buf;
    }
}

impl Node for String {
    fn gen_source(&self, _target: &CxxVersion) -> String {
        self.to_string()
    }
}
