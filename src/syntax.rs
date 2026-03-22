#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub name: String,
    pub signature: SurfaceTerm,
    pub body: SurfaceTerm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binder {
    pub name: String,
    pub ty: Box<SurfaceTerm>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceTerm {
    Type,
    Var(String),
    Lambda {
        binder: Binder,
        body: Box<SurfaceTerm>,
    },
    App(Box<SurfaceTerm>, Box<SurfaceTerm>),
    Arrow {
        binder: Option<Binder>,
        domain: Box<SurfaceTerm>,
        codomain: Box<SurfaceTerm>,
    },
}
