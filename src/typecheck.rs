use std::{collections::HashMap, fmt};

use crate::core::{self, Definition, Module, Term};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedModule {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeError {
    message: String,
}

impl TypeError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TypeError {}

#[derive(Debug, Clone, Default)]
struct Context {
    locals: Vec<Term>,
    globals: HashMap<String, (Term, Term)>,
}

pub fn check_module(module: &Module) -> Result<CheckedModule, TypeError> {
    let mut context = Context::default();

    for definition in &module.definitions {
        let ty_ty = infer(&context, &definition.ty)?;
        if !is_convertible(&context, &ty_ty, &Term::Type) {
            return Err(TypeError::new(format!(
                "definition '{}' annotation does not have type Type",
                definition.name
            )));
        }

        check(&context, &definition.body, &definition.ty)?;

        context.globals.insert(
            definition.name.clone(),
            (definition.ty.clone(), definition.body.clone()),
        );
    }

    Ok(CheckedModule {
        definitions: module.definitions.clone(),
    })
}

fn check(context: &Context, term: &Term, expected: &Term) -> Result<(), TypeError> {
    let expected = normalize_in(context, expected);

    match (term, &expected) {
        (
            Term::Lam {
                name,
                ty,
                body,
            },
            Term::Pi {
                domain, codomain, ..
            },
        ) => {
            if !is_convertible(context, ty, domain) {
                return Err(TypeError::new(format!(
                    "lambda binder type mismatch: expected {}, found {}",
                    domain.pretty(),
                    ty.pretty()
                )));
            }

            let mut next = context.clone();
            next.locals.push(core::shift(domain, 1));
            check(&next, body, codomain).map_err(|error| {
                TypeError::new(format!(
                    "while checking lambda binder '{name}': {}",
                    error
                ))
            })
        }
        _ => {
            let inferred = infer(context, term)?;
            if is_convertible(context, &inferred, &expected) {
                Ok(())
            } else {
                Err(TypeError::new(format!(
                    "type mismatch: expected {}, found {}",
                    expected.pretty(),
                    inferred.pretty()
                )))
            }
        }
    }
}

fn infer(context: &Context, term: &Term) -> Result<Term, TypeError> {
    match term {
        Term::Type => Ok(Term::Type),
        Term::Var(index) => context
            .locals
            .iter()
            .rev()
            .nth(*index)
            .cloned()
            .ok_or_else(|| TypeError::new(format!("unbound variable #{index}"))),
        Term::Global(name) => context
            .globals
            .get(name)
            .map(|(ty, _)| ty.clone())
            .ok_or_else(|| TypeError::new(format!("unknown global '{name}'"))),
        Term::Pi {
            name: _,
            domain,
            codomain,
        } => {
            check(context, domain, &Term::Type)?;

            let mut next = context.clone();
            next.locals.push(core::shift(domain, 1));
            check(&next, codomain, &Term::Type)?;

            Ok(Term::Type)
        }
        Term::Lam { name, ty, body } => {
            check(context, ty, &Term::Type)?;

            let mut next = context.clone();
            next.locals.push(core::shift(ty, 1));
            let body_ty = infer(&next, body)?;

            Ok(Term::Pi {
                name: name.clone(),
                domain: ty.clone(),
                codomain: Box::new(body_ty),
            })
        }
        Term::App(function, argument) => {
            let function_ty = normalize_in(context, &infer(context, function)?);

            match function_ty {
                Term::Pi {
                    domain, codomain, ..
                } => {
                    check(context, argument, &domain)?;
                    Ok(core::substitute_top(argument, &codomain))
                }
                other => Err(TypeError::new(format!(
                    "cannot apply non-function term with type {}",
                    other.pretty()
                ))),
            }
        }
    }
}

fn is_convertible(context: &Context, left: &Term, right: &Term) -> bool {
    normalize_in(context, left) == normalize_in(context, right)
}

fn normalize_in(context: &Context, term: &Term) -> Term {
    match term {
        Term::Type => Term::Type,
        Term::Var(index) => Term::Var(*index),
        Term::Global(name) => {
            if let Some((_, body)) = context.globals.get(name) {
                normalize_in(context, body)
            } else {
                Term::Global(name.clone())
            }
        }
        Term::Pi {
            name,
            domain,
            codomain,
        } => Term::Pi {
            name: name.clone(),
            domain: Box::new(normalize_in(context, domain)),
            codomain: Box::new(normalize_in(context, codomain)),
        },
        Term::Lam { name, ty, body } => Term::Lam {
            name: name.clone(),
            ty: Box::new(normalize_in(context, ty)),
            body: Box::new(normalize_in(context, body)),
        },
        Term::App(function, argument) => {
            let normalized_function = normalize_in(context, function);
            let normalized_argument = normalize_in(context, argument);

            if let Term::Lam { body, .. } = normalized_function {
                normalize_in(context, &core::substitute_top(&normalized_argument, &body))
            } else {
                Term::App(
                    Box::new(normalized_function),
                    Box::new(normalized_argument),
                )
            }
        }
    }
}
