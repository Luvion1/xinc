//! Statement code generation.
//!
//! Converts each [`Statement`] variant into a readable textual
//! representation. This module is the human-facing emitter: control
//! flow and function declarations now emit well-formed Xin syntax
//! instead of surfacing [`CodegenError::InvalidStatement`].
//!
//! # Supported variants
//!
//! | Variant | Output |
//! |---------|--------|
//! | [`Statement::Let`] | `let <name> = <expr>;` |
//! | [`Statement::Expr`] | `<expr>;` |
//! | [`Statement::If`] | `if (<cond>) { <then> } else { <else> }` |
//! | [`Statement::While`] | `while (<cond>) { <body> }` |
//! | [`Statement::Return`] | `return <expr?>` |
//! | [`Statement::Block`] | `{ <stmts> }` |
//! | [`Statement::Fn`] | `fn <name>(<params>) <body>` |
//! | [`Statement::Assign`] | `<target> = <value>;` |

use super::expr::generate_expression;
use crate::CodegenError;
use xin_ast::{Statement, Type, BuiltinType};

/// Generate a textual representation of a single statement.
///
/// # Errors
///
/// [`CodegenError::Hir(HirError)`] if lowering encounters code that is
/// not yet supported by the textual emitter.
pub fn generate_statement(stmt: &Statement) -> Result<String, CodegenError> {
    Ok(match stmt {
        Statement::Let { name, value, .. } => {
            let val = generate_expression(value)?;
            format!("let {name} = {val};")
        }
        Statement::Expr(e) => {
            format!("{};", generate_expression(e)?)
        }
        Statement::If { cond, then, r#else } => {
            let cond = generate_expression(cond)?;
            let then_body = block(then)?;
            let else_body = match r#else {
                Some(e) => format!(" else {}", generate_statement(e)?),
                None => String::new(),
            };
            format!("if ({cond}) {then_body}{else_body}")
        }
        Statement::While { cond, body } => {
            let cond = generate_expression(cond)?;
            let body = block(body)?;
            format!("while ({cond}) {body}")
        }
        Statement::Return(e) => match e {
            Some(expr) => format!("return {};", generate_expression(expr)?),
            None => "return;".to_string(),
        },
        Statement::Block(stmts) => block(stmts)?,
        Statement::Fn { name, params, body, ret_ty } => {
            let params = params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", ");
            let ret = ret_ty.as_ref().map_or(String::new(), |t| format!(": {}", type_str(t)));
            let body = block(body)?;
            format!("fn {name}({params}){ret} {body}")
        }
        Statement::Assign { target, value } => {
            let val = generate_expression(value)?;
            format!("{target} = {val};")
        }
    })
}

fn block(stmts: &[Statement]) -> Result<String, CodegenError> {
    let mut out = String::from("{\n");
    for s in stmts {
        out.push_str(&generate_statement(s)?);
        out.push('\n');
    }
    out.push('}');
    Ok(out)
}

fn type_str(t: &Type) -> String {
    match t {
        Type::Builtin(b) => match b {
            BuiltinType::I32 => "i32".into(),
            BuiltinType::I64 => "i64".into(),
            BuiltinType::F32 => "f32".into(),
            BuiltinType::F64 => "f64".into(),
            BuiltinType::Bool => "bool".into(),
            BuiltinType::Str => "str".into(),
        },
        Type::Named(n) => n.clone(),
    }
}
