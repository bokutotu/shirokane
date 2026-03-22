use std::collections::HashMap;

use crate::core::{self, Module, Term};

pub fn normalize_in_module(module: &Module, term: &Term) -> Term {
    let globals = module
        .definitions
        .iter()
        .map(|definition| (definition.name.clone(), definition.body.clone()))
        .collect::<HashMap<_, _>>();

    normalize(term, &globals)
}

fn normalize(term: &Term, globals: &HashMap<String, Term>) -> Term {
    match term {
        Term::Type => Term::Type,
        Term::Var(index) => Term::Var(*index),
        Term::Global(name) => globals
            .get(name)
            .map(|body| normalize(body, globals))
            .unwrap_or_else(|| Term::Global(name.clone())),
        Term::Pi {
            name,
            domain,
            codomain,
        } => Term::Pi {
            name: name.clone(),
            domain: Box::new(normalize(domain, globals)),
            codomain: Box::new(normalize(codomain, globals)),
        },
        Term::Lam { name, ty, body } => Term::Lam {
            name: name.clone(),
            ty: Box::new(normalize(ty, globals)),
            body: Box::new(normalize(body, globals)),
        },
        Term::App(function, argument) => {
            let normalized_function = normalize(function, globals);
            let normalized_argument = normalize(argument, globals);

            if let Term::Lam { body, .. } = normalized_function {
                normalize(&core::substitute_top(&normalized_argument, &body), globals)
            } else {
                Term::App(
                    Box::new(normalized_function),
                    Box::new(normalized_argument),
                )
            }
        }
    }
}
