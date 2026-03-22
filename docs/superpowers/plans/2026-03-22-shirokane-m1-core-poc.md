# Shirokane Milestone 1 Core PoC Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust proof-of-concept that parses a single Haskell-like `.sk` file, elaborates it into a tiny dependently typed core, typechecks the definitions, normalizes them, and prints stable output for simple annotated examples.

**Architecture:** Keep the parser/front-end separate from the semantic core. Parse Haskell-like syntax into a surface AST with names, elaborate into a tiny de Bruijn-indexed core, then typecheck and normalize that core before printing. This keeps the milestone-1 implementation small while avoiding variable-capture bugs in substitution-heavy code.

**Tech Stack:** Rust 2024, standard library, `cargo test`, integration tests under `tests/`

---

## Implementation Notes

- Follow `@superpowers/test-driven-development` for every task: write the failing test first, run it, implement the minimum code, re-run it, then refactor only while staying green.
- Use `@superpowers/verification-before-completion` before claiming the PoC is complete.
- When executing this plan in earnest, prefer `@superpowers/using-git-worktrees` before code changes so implementation happens in an isolated workspace.

## Language Assumptions Locked For Milestone 1

Use this exact source shape in tests and examples:

```haskell
id : (A : Type) -> A -> A
id = \(A : Type) -> \(x : A) -> x

typeId : Type -> Type
typeId = id Type
```

Assumptions:

- Single-file programs only
- Every top-level definition has a separate type signature and equation
- `->` is right-associative
- `(x : A) -> B` elaborates to a dependent `Pi`
- `A -> B` elaborates to a non-dependent `Pi`
- Lambda binders are parenthesized and annotated: `\(x : A) -> body`
- No recursion, modules, imports, or effects in milestone 1

## File Structure

- Create: `src/lib.rs` — library entrypoint re-exporting parser, elaborator, checker, evaluator, and pipeline helpers
- Create: `src/syntax.rs` — surface AST with names and source spans
- Create: `src/parser.rs` — tokenizer/parser for the milestone-1 Haskell-like subset
- Create: `src/core.rs` — tiny internal core calculus using de Bruijn indices plus pretty-print helpers
- Create: `src/elaborate.rs` — surface-to-core lowering and name resolution
- Create: `src/typecheck.rs` — core type inference/checking for the tiny calculus
- Create: `src/eval.rs` — normalization and beta-reduction for core terms
- Create: `src/pipeline.rs` — high-level `parse -> elaborate -> typecheck -> normalize -> render`
- Modify: `src/main.rs` — thin CLI wrapper around the pipeline
- Create: `tests/parser_surface.rs` — parser-focused tests
- Create: `tests/elaboration.rs` — elaboration/name-resolution tests
- Create: `tests/typecheck.rs` — typing tests for the smallest valid programs
- Create: `tests/normalize.rs` — normalization tests for beta-reduction
- Create: `tests/cli.rs` — end-to-end CLI tests using fixture files
- Create: `tests/fixtures/type_id.sk` — happy-path sample program for end-to-end testing

## Task 1: Parse The Smallest Haskell-Like Module

**Files:**
- Create: `src/lib.rs`
- Create: `src/syntax.rs`
- Create: `src/parser.rs`
- Test: `tests/parser_surface.rs`

- [ ] **Step 1: Write the failing parser test**

```rust
use shirokane::parser::parse_module;

#[test]
fn parses_typed_identity_definition() {
    let source = r#"id : (A : Type) -> A -> A
id = \(A : Type) -> \(x : A) -> x
"#;

    let module = parse_module(source).expect("module should parse");
    assert_eq!(module.definitions.len(), 1);
    assert_eq!(module.definitions[0].name, "id");
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --test parser_surface parses_typed_identity_definition -- --exact`
Expected: FAIL because `shirokane::parser::parse_module` and the syntax types do not exist yet

- [ ] **Step 3: Write the minimum parser implementation**

```rust
// src/syntax.rs
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
    Lambda { binder: Binder, body: Box<SurfaceTerm> },
    App(Box<SurfaceTerm>, Box<SurfaceTerm>),
    Arrow { binder: Option<Binder>, codomain: Box<SurfaceTerm> },
}
```

```rust
// src/parser.rs
pub fn parse_module(input: &str) -> Result<Module, ParseError> {
    // Minimal handwritten tokenizer + recursive descent parser
    // Supports identifiers, `Type`, `:`, `=`, `->`, `\\`, `(`, `)`, and newlines
}
```

- [ ] **Step 4: Run the parser test to verify it passes**

Run: `cargo test --test parser_surface parses_typed_identity_definition -- --exact`
Expected: PASS

- [ ] **Step 5: Commit the parser foundation**

```bash
git add src/lib.rs src/syntax.rs src/parser.rs tests/parser_surface.rs
git commit -m "feat: parse the first shirokane module"
```

## Task 2: Elaborate Surface Terms Into A Tiny Core

**Files:**
- Create: `src/core.rs`
- Create: `src/elaborate.rs`
- Modify: `src/lib.rs`
- Test: `tests/elaboration.rs`

- [ ] **Step 1: Write the failing elaboration test**

```rust
use shirokane::elaborate::elaborate_module;
use shirokane::parser::parse_module;

#[test]
fn elaborates_identity_signature_and_body() {
    let source = r#"id : (A : Type) -> A -> A
id = \(A : Type) -> \(x : A) -> x
"#;

    let surface = parse_module(source).unwrap();
    let core = elaborate_module(&surface).unwrap();

    assert_eq!(core.definitions.len(), 1);
    assert_eq!(core.definitions[0].name, "id");
    assert_eq!(core.definitions[0].ty.pretty(), "(A : Type) -> (x : A) -> A");
    assert_eq!(core.definitions[0].body.pretty(), "\\(A : Type) -> \\(x : A) -> x");
}
```

- [ ] **Step 2: Run the elaboration test to verify it fails**

Run: `cargo test --test elaboration elaborates_identity_signature_and_body -- --exact`
Expected: FAIL because the core representation and elaborator do not exist yet

- [ ] **Step 3: Write the minimum core and elaborator**

```rust
// src/core.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Type,
    Var(usize),
    Pi { name: String, domain: Box<Term>, codomain: Box<Term> },
    Lam { name: String, ty: Box<Term>, body: Box<Term> },
    App(Box<Term>, Box<Term>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub name: String,
    pub ty: Term,
    pub body: Term,
}
```

```rust
// src/elaborate.rs
pub fn elaborate_module(surface: &syntax::Module) -> Result<core::Module, ElabError> {
    // Resolve names into de Bruijn indices
    // Lower `(x : A) -> B` into `Pi`
    // Lower `A -> B` into anonymous `Pi`
    // Lower lambdas into annotated `Lam`
}
```

- [ ] **Step 4: Run the elaboration test to verify it passes**

Run: `cargo test --test elaboration elaborates_identity_signature_and_body -- --exact`
Expected: PASS

- [ ] **Step 5: Commit the core lowering step**

```bash
git add src/lib.rs src/core.rs src/elaborate.rs tests/elaboration.rs
git commit -m "feat: elaborate surface syntax into core terms"
```

## Task 3: Typecheck Annotated Core Definitions

**Files:**
- Create: `src/typecheck.rs`
- Modify: `src/core.rs`
- Modify: `src/elaborate.rs`
- Modify: `src/lib.rs`
- Test: `tests/typecheck.rs`

- [ ] **Step 1: Write the failing typechecking test**

```rust
use shirokane::elaborate::elaborate_module;
use shirokane::parser::parse_module;
use shirokane::typecheck::check_module;

#[test]
fn typechecks_identity_and_application_to_type() {
    let source = r#"id : (A : Type) -> A -> A
id = \(A : Type) -> \(x : A) -> x

typeId : Type -> Type
typeId = id Type
"#;

    let surface = parse_module(source).unwrap();
    let core = elaborate_module(&surface).unwrap();
    let checked = check_module(&core).expect("module should typecheck");

    assert_eq!(checked.definitions.len(), 2);
    assert_eq!(checked.definitions[1].ty.pretty(), "Type -> Type");
}
```

- [ ] **Step 2: Run the typechecking test to verify it fails**

Run: `cargo test --test typecheck typechecks_identity_and_application_to_type -- --exact`
Expected: FAIL because `check_module` does not exist and application typing is not implemented

- [ ] **Step 3: Write the minimum typechecker**

```rust
// src/typecheck.rs
pub fn check_module(module: &core::Module) -> Result<CheckedModule, TypeError> {
    // For each definition:
    // 1. infer/check that the annotation itself has type `Type`
    // 2. check the body against the annotation
    // 3. extend the global environment for later definitions
}

fn infer(ctx: &Context, term: &Term) -> Result<Term, TypeError> {
    // Type : Type is accepted for this PoC
    // Var looks up from local or global context
    // Pi checks domain/codomain are Type and returns Type
    // Lam requires expected type via check()
    // App infers function type and substitutes the argument into the codomain
}
```

- [ ] **Step 4: Run the typechecking test to verify it passes**

Run: `cargo test --test typecheck typechecks_identity_and_application_to_type -- --exact`
Expected: PASS

- [ ] **Step 5: Commit the typechecker**

```bash
git add src/lib.rs src/core.rs src/elaborate.rs src/typecheck.rs tests/typecheck.rs
git commit -m "feat: typecheck the milestone-one core"
```

## Task 4: Normalize Beta-Reducible Terms

**Files:**
- Create: `src/eval.rs`
- Modify: `src/core.rs`
- Modify: `src/lib.rs`
- Test: `tests/normalize.rs`

- [ ] **Step 1: Write the failing normalization test**

```rust
use shirokane::elaborate::elaborate_module;
use shirokane::eval::normalize;
use shirokane::parser::parse_module;

#[test]
fn normalizes_type_id_to_identity_function() {
    let source = r#"id : (A : Type) -> A -> A
id = \(A : Type) -> \(x : A) -> x

typeId : Type -> Type
typeId = id Type
"#;

    let surface = parse_module(source).unwrap();
    let core = elaborate_module(&surface).unwrap();
    let term = &core.definitions[1].body;

    assert_eq!(normalize(term).pretty(), "\\(x : Type) -> x");
}
```

- [ ] **Step 2: Run the normalization test to verify it fails**

Run: `cargo test --test normalize normalizes_type_id_to_identity_function -- --exact`
Expected: FAIL because `normalize` does not exist yet

- [ ] **Step 3: Write the minimum evaluator**

```rust
// src/eval.rs
pub fn normalize(term: &Term) -> Term {
    match term {
        Term::Type | Term::Var(_) => term.clone(),
        Term::Pi { name, domain, codomain } => Term::Pi {
            name: name.clone(),
            domain: Box::new(normalize(domain)),
            codomain: Box::new(normalize(codomain)),
        },
        Term::Lam { name, ty, body } => Term::Lam {
            name: name.clone(),
            ty: Box::new(normalize(ty)),
            body: Box::new(normalize(body)),
        },
        Term::App(fun, arg) => match normalize(fun) {
            Term::Lam { body, .. } => normalize(&substitute_top(&normalize(arg), &body)),
            normalized_fun => Term::App(Box::new(normalized_fun), Box::new(normalize(arg))),
        },
    }
}
```

- [ ] **Step 4: Run the normalization test to verify it passes**

Run: `cargo test --test normalize normalizes_type_id_to_identity_function -- --exact`
Expected: PASS

- [ ] **Step 5: Commit the evaluator**

```bash
git add src/lib.rs src/core.rs src/eval.rs tests/normalize.rs
git commit -m "feat: normalize beta-reducible core terms"
```

## Task 5: Add The Pipeline And CLI

**Files:**
- Create: `src/pipeline.rs`
- Modify: `src/lib.rs`
- Modify: `src/main.rs`
- Create: `tests/fixtures/type_id.sk`
- Test: `tests/cli.rs`

- [ ] **Step 1: Write the failing CLI test**

```rust
use std::process::Command;

#[test]
fn cli_prints_normalized_definitions_for_fixture() {
    let output = Command::new(env!("CARGO_BIN_EXE_shirokane"))
        .arg("tests/fixtures/type_id.sk")
        .output()
        .expect("binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("typeId : Type -> Type"));
    assert!(stdout.contains("\\(x : Type) -> x"));
}
```

- [ ] **Step 2: Run the CLI test to verify it fails**

Run: `cargo test --test cli cli_prints_normalized_definitions_for_fixture -- --exact`
Expected: FAIL because the binary still prints `Hello, world!`

- [ ] **Step 3: Write the minimum pipeline and CLI**

```rust
// src/pipeline.rs
pub fn run_source(source: &str) -> Result<String, ShirokaneError> {
    let surface = parser::parse_module(source)?;
    let core = elaborate::elaborate_module(&surface)?;
    let checked = typecheck::check_module(&core)?;

    let rendered = checked
        .definitions
        .iter()
        .map(|definition| {
            let normalized = eval::normalize(&definition.body);
            format!("{} : {}\n{} = {}", definition.name, definition.ty.pretty(), definition.name, normalized.pretty())
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    Ok(rendered)
}
```

```rust
// src/main.rs
fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: shirokane <file.sk>");
        std::process::exit(2);
    });

    let source = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        eprintln!("failed to read {path}: {err}");
        std::process::exit(1);
    });

    match shirokane::pipeline::run_source(&source) {
        Ok(output) => println!("{output}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
```

- [ ] **Step 4: Run the CLI test to verify it passes**

Run: `cargo test --test cli cli_prints_normalized_definitions_for_fixture -- --exact`
Expected: PASS

- [ ] **Step 5: Commit the user-visible PoC**

```bash
git add src/lib.rs src/pipeline.rs src/main.rs tests/fixtures/type_id.sk tests/cli.rs
git commit -m "feat: run the shirokane milestone-one pipeline from the cli"
```

## Final Verification

- [ ] Run the focused suite:

```bash
cargo test --test parser_surface
cargo test --test elaboration
cargo test --test typecheck
cargo test --test normalize
cargo test --test cli
```

Expected: all targeted tests PASS

- [ ] Run the full suite:

```bash
cargo test
```

Expected: all tests PASS

- [ ] Smoke-test the binary manually:

```bash
cargo run -- tests/fixtures/type_id.sk
```

Expected output contains:

```text
id : (A : Type) -> A -> A
typeId : Type -> Type
\(x : Type) -> x
```

## Out Of Scope For This Plan

- Built-in base data types like `Nat` or `Bool`
- Pattern matching
- User-defined algebraic data types
- Effects syntax or semantics
- Hefty-style higher-order effects
- Code generation to C or JavaScript
- Recursion or termination checking
- Multi-file modules or imports
