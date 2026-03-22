# Shirokane Milestone 1 Core Design

**Project:** Shirokane

**Status:** Draft approved in chat, pending final user review

## Goal

Build the smallest possible Rust proof-of-concept for Shirokane as a dependently typed language with Haskell-like syntax. Milestone 1 focuses only on parsing, elaboration, typechecking, and normalization for a tiny core calculus.

## Non-Goals

- No Hefty-style higher-order effects in milestone 1
- No code generation to C or JavaScript in milestone 1
- No recursion
- No module/import system
- No package/project system
- No aggressive type inference
- No advanced diagnostics work

These are intentionally deferred so the first prototype stays as small and testable as possible.

## Milestone Boundaries

### Milestone 1

- Single-file `.sk` programs only
- Haskell-like surface syntax from day one
- Explicit type annotations required
- Tiny internal dependent core
- Typecheck and normalize source files
- First guaranteed demo: typed identity and constant-style functions

### Milestone 2

- Extend the core toward Hefty-style higher-order algebraic effects

### Later Milestones

- User-defined data types
- Pattern matching beyond the smallest builtins
- Structured recursion
- Code generation to C or JavaScript
- Better diagnostics
- Modules and project layout

## Architecture

The Rust PoC uses a two-layer design:

1. A Haskell-like surface language used only for parsing and user-facing syntax
2. A very small internal core calculus used for elaboration, typechecking, and normalization

Pipeline:

`source (.sk)` -> `surface AST` -> `elaboration` -> `core calculus` -> `typecheck` -> `normalize` -> `print`

This keeps the milestone-1 implementation small while preserving a clean foundation for later extensions. The surface syntax can evolve without forcing the internal semantics to become complicated too early.

## Core Calculus Scope

Milestone 1 core terms:

- `Type`
- variables
- dependent function types (`Pi`)
- lambda
- application

The first version should avoid extra constructs unless they are required to support the smallest working examples.

## Surface Language Scope

Milestone 1 surface syntax should feel Haskell-like, but only for a very small subset:

- top-level typed definitions
- lambda expressions
- function application
- names and binders

The surface language is intentionally narrow. It should be just rich enough to express the first checked examples while elaborating into the tiny core.

## Type System Strategy

Milestone 1 should prefer explicitness over convenience:

- require top-level type annotations
- require explicit lambda binder annotations where needed by the implementation
- avoid ambitious inference or elaboration heuristics

The purpose of the first prototype is correctness and clarity, not surface-language sophistication.

## Execution Model

The first executable should:

1. Read a single `.sk` file
2. Parse it
3. Elaborate it into the core
4. Typecheck all definitions
5. Normalize selected definitions or all simple definitions
6. Print the checked type and normalized term

Milestone 1 does not need a REPL or runtime-oriented `main` story.

## Component Layout

Planned Rust modules:

- `src/main.rs` — CLI entrypoint
- `src/syntax.rs` — surface AST
- `src/parser.rs` — parser for Haskell-like syntax
- `src/core.rs` — tiny internal core calculus
- `src/elaborate.rs` — surface-to-core lowering
- `src/typecheck.rs` — core typechecker
- `src/eval.rs` — normalization/evaluation
- `tests/` — parser, checker, evaluator, and end-to-end tests

Each file should keep a single clear responsibility so the PoC stays easy to reason about.

## Data Flow

For a single-file run:

1. CLI loads the source file
2. Parser builds a surface AST
3. Elaboration lowers to core terms
4. Typechecker validates core definitions against annotations
5. Evaluator normalizes terms
6. Pretty-printer emits human-readable results

For the first success case, elaboration should be shallow and mostly structural.

## Error Handling

Milestone 1 diagnostics should be simple but understandable:

- parse errors with location and a short message
- elaboration errors for unknown names, duplicates, or missing required annotations
- type errors for mismatches and invalid application

Fancy diagnostics are out of scope for now.

## Testing Strategy

The prototype should be built test-first in small steps:

- parser tests for signatures and tiny function definitions
- typechecker tests for small annotated examples
- evaluator tests for normalization of simple applications
- end-to-end CLI tests using tiny `.sk` files

The first required demo should be the smallest meaningful example, such as identity and constant-style definitions.

## Success Criteria

Milestone 1 is successful when:

- a single `.sk` file parses successfully
- the file elaborates into the tiny core
- typed identity/constant-style examples typecheck
- normalization works for the first examples
- the CLI prints stable, understandable output

## Open Follow-Up Items

- Exact binder syntax for the Haskell-like surface language
- Whether milestone 1 needs any built-in base types immediately or can begin with only the tiniest pure core examples
- Output format for normalized terms and checked types

## User Choices Captured

- Rust implementation for the PoC
- Milestone 1 should be the simplest possible goal
- Haskell-like syntax from day one
- Explicit annotations over inference
- Single-file programs only
- No recursion in milestone 1
- First executable should typecheck and normalize a file
- Hefty-style higher-order effects are a milestone-2 goal
- Code generation to C or JavaScript is deferred until later
