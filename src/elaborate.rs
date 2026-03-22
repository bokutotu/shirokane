use std::fmt;

use crate::{
    core::{Definition, Module, Term},
    syntax::{self, SurfaceTerm},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElabError {
    message: String,
}

impl ElabError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ElabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ElabError {}

pub fn elaborate_module(surface: &syntax::Module) -> Result<Module, ElabError> {
    let globals = surface
        .definitions
        .iter()
        .map(|definition| definition.name.clone())
        .collect::<Vec<_>>();

    let definitions = surface
        .definitions
        .iter()
        .map(|definition| elaborate_definition(definition, &globals))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Module { definitions })
}

fn elaborate_definition(
    definition: &syntax::Definition,
    globals: &[String],
) -> Result<Definition, ElabError> {
    let mut env = Vec::new();
    let ty = elaborate_term(&definition.signature, globals, &mut env)?;

    let mut env = Vec::new();
    let body = elaborate_term(&definition.body, globals, &mut env)?;

    Ok(Definition {
        name: definition.name.clone(),
        ty,
        body,
    })
}

fn elaborate_term(
    term: &SurfaceTerm,
    globals: &[String],
    env: &mut Vec<String>,
) -> Result<Term, ElabError> {
    match term {
        SurfaceTerm::Type => Ok(Term::Type),
        SurfaceTerm::Var(name) => {
            if let Some(index) = env.iter().rev().position(|candidate| candidate == name) {
                Ok(Term::Var(index))
            } else if globals.iter().any(|candidate| candidate == name) {
                Ok(Term::Global(name.clone()))
            } else {
                Err(ElabError::new(format!("unknown variable '{name}'")))
            }
        }
        SurfaceTerm::Lambda { binder, body } => {
            let ty = elaborate_term(&binder.ty, globals, env)?;
            env.push(binder.name.clone());
            let body = elaborate_term(body, globals, env)?;
            env.pop();
            Ok(Term::Lam {
                name: binder.name.clone(),
                ty: Box::new(ty),
                body: Box::new(body),
            })
        }
        SurfaceTerm::App(function, argument) => Ok(Term::App(
            Box::new(elaborate_term(function, globals, env)?),
            Box::new(elaborate_term(argument, globals, env)?),
        )),
        SurfaceTerm::Arrow {
            binder,
            domain,
            codomain,
        } => {
            let domain = elaborate_term(domain, globals, env)?;
            let name = binder
                .as_ref()
                .map(|named| named.name.clone())
                .unwrap_or_else(|| "_".to_string());

            if let Some(named) = binder {
                env.push(named.name.clone());
            }

            let mut codomain = elaborate_term(codomain, globals, env)?;

            if binder.is_some() {
                env.pop();
            } else {
                codomain = crate::core::shift(&codomain, 1);
            }

            Ok(Term::Pi {
                name,
                domain: Box::new(domain),
                codomain: Box::new(codomain),
            })
        }
    }
}
