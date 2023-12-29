use std::fmt::Display;
use log::warn;
use super::ast::{Node, FunctionDeclaration, FunctionCall};

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
        write!(f, ") {{\n")?;
        for node in self.body {
            write!(f, "{}\n", node)?;
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

