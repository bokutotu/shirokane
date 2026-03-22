#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub name: String,
    pub ty: Term,
    pub body: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Type,
    Var(usize),
    Global(String),
    Pi {
        name: String,
        domain: Box<Term>,
        codomain: Box<Term>,
    },
    Lam {
        name: String,
        ty: Box<Term>,
        body: Box<Term>,
    },
    App(Box<Term>, Box<Term>),
}

impl Term {
    pub fn pretty(&self) -> String {
        self.pretty_with_names(&mut Vec::new())
    }

    fn pretty_with_names(&self, names: &mut Vec<String>) -> String {
        match self {
            Self::Type => "Type".to_string(),
            Self::Var(index) => names
                .iter()
                .rev()
                .nth(*index)
                .cloned()
                .unwrap_or_else(|| format!("#{index}")),
            Self::Global(name) => name.clone(),
            Self::Pi {
                name,
                domain,
                codomain,
            } => {
                let domain_pretty = domain.pretty_with_names(names);
                names.push(name.clone());
                let codomain_pretty = codomain.pretty_with_names(names);
                names.pop();
                format!("({name} : {domain_pretty}) -> {codomain_pretty}")
            }
            Self::Lam { name, ty, body } => {
                let ty_pretty = ty.pretty_with_names(names);
                names.push(name.clone());
                let body_pretty = body.pretty_with_names(names);
                names.pop();
                format!("\\({name} : {ty_pretty}) -> {body_pretty}")
            }
            Self::App(function, argument) => {
                let function_pretty = function.pretty_with_names(names);
                let argument_pretty = argument.pretty_with_names(names);
                format!("{function_pretty} {argument_pretty}")
            }
        }
    }
}

pub fn shift(term: &Term, amount: isize) -> Term {
    shift_from(term, amount, 0)
}

fn shift_from(term: &Term, amount: isize, cutoff: usize) -> Term {
    match term {
        Term::Type => Term::Type,
        Term::Var(index) => {
            if *index >= cutoff {
                let shifted = (*index as isize) + amount;
                Term::Var(shifted as usize)
            } else {
                Term::Var(*index)
            }
        }
        Term::Global(name) => Term::Global(name.clone()),
        Term::Pi {
            name,
            domain,
            codomain,
        } => Term::Pi {
            name: name.clone(),
            domain: Box::new(shift_from(domain, amount, cutoff)),
            codomain: Box::new(shift_from(codomain, amount, cutoff + 1)),
        },
        Term::Lam { name, ty, body } => Term::Lam {
            name: name.clone(),
            ty: Box::new(shift_from(ty, amount, cutoff)),
            body: Box::new(shift_from(body, amount, cutoff + 1)),
        },
        Term::App(function, argument) => Term::App(
            Box::new(shift_from(function, amount, cutoff)),
            Box::new(shift_from(argument, amount, cutoff)),
        ),
    }
}

pub fn substitute_top(value: &Term, body: &Term) -> Term {
    shift(&substitute(body, &shift(value, 1), 0), -1)
}

fn substitute(term: &Term, value: &Term, depth: usize) -> Term {
    match term {
        Term::Type => Term::Type,
        Term::Var(index) => {
            if *index == depth {
                shift(value, depth as isize)
            } else {
                Term::Var(*index)
            }
        }
        Term::Global(name) => Term::Global(name.clone()),
        Term::Pi {
            name,
            domain,
            codomain,
        } => Term::Pi {
            name: name.clone(),
            domain: Box::new(substitute(domain, value, depth)),
            codomain: Box::new(substitute(codomain, value, depth + 1)),
        },
        Term::Lam { name, ty, body } => Term::Lam {
            name: name.clone(),
            ty: Box::new(substitute(ty, value, depth)),
            body: Box::new(substitute(body, value, depth + 1)),
        },
        Term::App(function, argument) => Term::App(
            Box::new(substitute(function, value, depth)),
            Box::new(substitute(argument, value, depth)),
        ),
    }
}
