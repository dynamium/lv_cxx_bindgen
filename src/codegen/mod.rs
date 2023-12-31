pub mod ast;

use ast::{FunctionCall, FunctionDeclaration, Node};
use log::{warn, debug};
use std::fmt::Display;

use crate::{group::Namespace, conf::CXXVersion};

use self::ast::NamespaceDeclaration;

// TODO: convert this code to proper code generator functions
impl Node for FunctionDeclaration<'_> {}
impl Display for FunctionDeclaration<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}(", self.return_type, self.identifier)?;
        for arg in self.args {
            if let Some(ident) = &arg.identifier {
                write!(f, "{} {},", arg.kind, ident)?;
            } else {
                warn!("Function argument doesn't have an identifier, that's a problem'");
            }
        }
        writeln!(f, ") {{")?;
        for node in self.body {
            writeln!(f, "{}", node)?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}

impl Node for FunctionCall<'_> {}
impl Display for FunctionCall<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for component in self.path {
            write!(f, "{component}::")?;
        }
        write!(f, "{}(", self.identifier)?;
        for arg in self.args {
            write!(f, "{arg}")?;
        }
        write!(f, ");")?;

        Ok(())
    }
}

impl NamespaceDeclaration<'_> {
    pub fn gen_source(&self, target: CXXVersion) -> String {
        let mut buf = String::new();
        buf.push_str(&make_code_block(&format!("namespace {}", self.identifier), || {
            let mut buf = String::new();

            for func in &self.members {
                let mut header = String::new();
                header.push_str(&format!("{} {}(", func.return_type, func.identifier));

                for (i, arg) in func.args.into_iter().enumerate() {
                    header.push_str(&format!("{} {}", arg.clone().kind, arg.clone().identifier.unwrap()));
                    if i != func.args.len() - 1 { // fixes trailing comma
                        header.push(',');
                    }
                }

                header.push_str(")");

                buf.push_str(&make_code_block(&header, || {
                    "".to_string()
                        // todo!()
                }))
            }

            buf
        }));

        buf
    }
}

fn make_code_block<F: Fn() -> String>(header: &str, content: F) -> String {
    let mut buf = format!("{header} {{\n");
    buf.push_str(&content());
    buf.push('}');
    buf.push('\n');
    return buf;
}
