use crate::{elaborate, eval, parser, typecheck};

pub fn run_source(source: &str) -> Result<String, String> {
    let surface = parser::parse_module(source).map_err(|error| error.to_string())?;
    let core = elaborate::elaborate_module(&surface).map_err(|error| error.to_string())?;
    let checked = typecheck::check_module(&core).map_err(|error| error.to_string())?;

    let rendered = checked
        .definitions
        .iter()
        .map(|definition| {
            let normalized = eval::normalize_in_module(&core, &definition.body);
            format!(
                "{} : {}\n{} = {}",
                definition.name,
                definition.ty.pretty(),
                definition.name,
                normalized.pretty()
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    Ok(rendered)
}
