//! Phase 1 authoring model. No implicit RPG concepts or executable rule runtime.
mod constraints;
mod topology;
mod model;
mod parse;
mod validation;

pub use model::*;
pub use parse::{load, MAX_INPUT_BYTES};
pub use validation::{validate, Diagnostic, ReferenceEdge};

/// Validates before serialization. Output is Ludeforge v1 normalized JSON, not JCS.
pub fn normalize(project: &Project) -> Result<Vec<u8>, Vec<Diagnostic>> {
    validate(project)?;
    let mut p = project.clone();
    p.types.sort_by(|a, b| a.id.cmp(&b.id));
    for t in &mut p.types {
        t.fields.sort_by(|a, b| a.id.cmp(&b.id));
        for f in &mut t.fields {
            f.options.sort_by(|a, b| a.id.cmp(&b.id));
        }
    }
    p.constraints.sort_by(|a, b| a.id().cmp(b.id()));
    p.objects.sort_by(|a, b| a.id.cmp(&b.id));
    p.relation_types.sort_by(|a, b| a.id.cmp(&b.id));
    p.relations.sort_by(|a, b| a.id.cmp(&b.id));
    let mut bytes = serde_json::to_vec_pretty(&p)
        .map_err(|e| vec![Diagnostic::new("SERIALIZE", "", e.to_string())])?;
    bytes.push(b'\n');
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(vec![Diagnostic::new(
            "FORMAT_SIZE",
            "",
            "normalized project exceeds input size limit",
        )]);
    }
    Ok(bytes)
}

/// Returns a sorted, validated index of explicit references in authored data.
pub fn references(project: &Project) -> Result<Vec<ReferenceEdge>, Vec<Diagnostic>> {
    validate(project)?;
    Ok(validation::reference_edges(project))
}
