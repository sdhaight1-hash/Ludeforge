use crate::{Direction, Field, Kind, Project, Value, SAFE_INTEGER};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

const MAX_RECORDS: usize = 100_000;
const MAX_DIAGNOSTICS: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Diagnostic {
    pub path: String,
    pub code: &'static str,
    pub message: String,
}

impl Diagnostic {
    pub fn new(code: &'static str, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            path: path.into(),
            message: message.into(),
        }
    }
}

#[derive(Default)]
struct Errors(Vec<Diagnostic>);

impl Errors {
    fn add(&mut self, code: &'static str, path: &str, message: impl Into<String>) {
        if self.0.len() < MAX_DIAGNOSTICS {
            self.0.push(Diagnostic::new(code, path, message));
        }
    }

    fn full(&self) -> bool {
        self.0.len() >= MAX_DIAGNOSTICS
    }

    fn finish(mut self) -> Result<(), Vec<Diagnostic>> {
        if self.0.is_empty() {
            return Ok(());
        }
        if self.full() {
            self.0.push(Diagnostic::new(
                "DIAGNOSTIC_LIMIT",
                "",
                "validation stopped or diagnostics capped at 256",
            ));
        }
        self.0.sort();
        Err(self.0)
    }
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id.as_bytes()[0].is_ascii_alphabetic()
        && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

fn identity<'a>(ids: &mut BTreeSet<&'a str>, errors: &mut Errors, id: &'a str, path: &str) {
    if !valid_id(id) {
        errors.add("ID_INVALID", path, "expected ASCII id, 1–64 characters");
    } else if !ids.insert(id) {
        errors.add("ID_DUPLICATE", path, "identifier already declared");
    }
}

fn label(errors: &mut Errors, s: &str, path: &str) {
    if s.trim().is_empty() {
        errors.add("LABEL_EMPTY", path, "label must not be blank");
    }
}

fn safe(n: i64) -> bool {
    (-SAFE_INTEGER..=SAFE_INTEGER).contains(&n)
}

fn field_shape(f: &Field, errors: &mut Errors, path: &str) {
    let valid = match f.kind {
        Kind::Int => matches!((f.min, f.max), (Some(a), Some(b)) if safe(a) && safe(b) && a <= b)
            && f.target.is_none()
            && f.options.is_empty(),
        Kind::Ref => f.target.is_some() && f.min.is_none() && f.max.is_none() && f.options.is_empty(),
        Kind::Enum => !f.options.is_empty() && f.target.is_none() && f.min.is_none() && f.max.is_none(),
        Kind::Bool | Kind::Text => f.target.is_none()
            && f.min.is_none()
            && f.max.is_none()
            && f.options.is_empty(),
    };
    if !valid {
        errors.add("FIELD_SCHEMA", path, "incompatible or missing kind-specific declaration");
    }
}

fn ptr(s: &str) -> String {
    s.replace('~', "~0").replace('/', "~1")
}

/// Structural-semantic checks independent of specific game categories.
pub fn validate(p: &Project) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Errors::default();
    if p.format_version != 1 {
        errors.add("FORMAT_VERSION", "/format_version", "expected format_version 1");
    }
    if !p.rules.is_empty() {
        errors.add("RULE_UNSUPPORTED", "/rules", "v1 authoring kernel requires empty rules");
    }
    if !valid_id(&p.project_id) {
        errors.add("ID_INVALID", "/project_id", "invalid project identifier");
    }
    label(&mut errors, &p.label, "/label");
    let mut count = p.types.len().saturating_add(p.objects.len())
        .saturating_add(p.relation_types.len()).saturating_add(p.relations.len())
        .saturating_add(p.rules.len());
    for t in &p.types {
        count = count.saturating_add(t.fields.len());
        for f in &t.fields {
            count = count.saturating_add(f.options.len());
        }
    }
    for o in &p.objects {
        count = count.saturating_add(o.values.len());
    }
    if count > MAX_RECORDS {
        errors.add("FORMAT_LIMIT", "", "project exceeds 100000 logical records and values");
        return errors.finish();
    }
    let mut ids = BTreeSet::new();
    let mut types = BTreeMap::new();
    for (i, t) in p.types.iter().enumerate() {
        let path = format!("/types/{i}");
        identity(&mut ids, &mut errors, &t.id, &format!("{path}/id"));
        label(&mut errors, &t.label, &format!("{path}/label"));
        types.insert(t.id.as_str(), t);
        for (j, f) in t.fields.iter().enumerate() {
            let fp = format!("{path}/fields/{j}");
            identity(&mut ids, &mut errors, &f.id, &format!("{fp}/id"));
            label(&mut errors, &f.label, &format!("{fp}/label"));
            field_shape(f, &mut errors, &fp);
            for (k, opt) in f.options.iter().enumerate() {
                let op = format!("{fp}/options/{k}");
                identity(&mut ids, &mut errors, &opt.id, &format!("{op}/id"));
                label(&mut errors, &opt.label, &format!("{op}/label"));
            }
        }
    }
    let mut objects = BTreeMap::new();
    for (i, o) in p.objects.iter().enumerate() {
        let path = format!("/objects/{i}");
        identity(&mut ids, &mut errors, &o.id, &format!("{path}/id"));
        label(&mut errors, &o.label, &format!("{path}/label"));
        objects.insert(o.id.as_str(), o);
    }
    let mut relation_types = BTreeMap::new();
    for (i, t) in p.relation_types.iter().enumerate() {
        let path = format!("/relation_types/{i}");
        identity(&mut ids, &mut errors, &t.id, &format!("{path}/id"));
        label(&mut errors, &t.label, &format!("{path}/label"));
        relation_types.insert(t.id.as_str(), t);
    }
    for (i, r) in p.relations.iter().enumerate() {
        identity(&mut ids, &mut errors, &r.id, &format!("/relations/{i}/id"));
    }
    // Invalid identities or field shapes make semantic lookups ambiguous. Fail closed.
    if !errors.0.is_empty() {
        return errors.finish();
    }
    let mut fields = BTreeMap::new();
    let mut required = BTreeMap::new();
    let mut options = BTreeMap::new();
    for (i, t) in p.types.iter().enumerate() {
        let mut map = BTreeMap::new();
        let mut req = Vec::new();
        for (j, f) in t.fields.iter().enumerate() {
            map.insert(f.id.as_str(), f);
            if f.required {
                req.push(f.id.as_str());
            }
            if f.kind == Kind::Enum {
                options.insert(f.id.as_str(), f.options.iter().map(|o| o.id.as_str()).collect::<BTreeSet<_>>());
            }
            if let Some(target) = &f.target {
                if !types.contains_key(target.as_str()) {
                    errors.add("REF_MISSING", &format!("/types/{i}/fields/{j}/target"), "reference target type not found");
                }
            }
        }
        fields.insert(t.id.as_str(), map);
        required.insert(t.id.as_str(), req);
    }
    for (i, o) in p.objects.iter().enumerate() {
        if errors.full() {
            return errors.finish();
        }
        let path = format!("/objects/{i}");
        let Some(schema) = fields.get(o.type_id.as_str()) else {
            errors.add("REF_MISSING", &format!("{path}/type"), "object type not found");
            continue;
        };
        for name in &required[o.type_id.as_str()] {
            if !o.values.contains_key(*name) {
                errors.add("FIELD_REQUIRED", &format!("{path}/values/{}", ptr(name)), "required field missing");
            }
            if errors.full() {
                return errors.finish();
            }
        }
        for (name, value) in &o.values {
            let vp = format!("{path}/values/{}", ptr(name));
            let Some(field) = schema.get(name.as_str()) else {
                errors.add("FIELD_UNKNOWN", &vp, "field not declared on object type");
                continue;
            };
            if field.kind != value.kind() {
                errors.add("VALUE_KIND", &vp, "value kind differs from schema");
                continue;
            }
            match value {
                Value::Int(n) => {
                    if !safe(*n) || *n < field.min.unwrap_or(-SAFE_INTEGER) || *n > field.max.unwrap_or(SAFE_INTEGER) {
                        errors.add("VALUE_RANGE", &vp, "integer outside declared range");
                    }
                }
                Value::Enum(id) => {
                    if !options[field.id.as_str()].contains(id.as_str()) {
                        errors.add("ENUM_OPTION", &vp, "enum option not declared on field");
                    }
                }
                Value::Ref(id) => match objects.get(id.as_str()) {
                    None => errors.add("REF_MISSING", &vp, "referenced object not found"),
                    Some(target) if Some(target.type_id.as_str()) != field.target.as_deref() => {
                        errors.add("REF_TYPE", &vp, "referenced object has wrong type");
                    }
                    _ => {}
                },
                Value::Bool(_) | Value::Text(_) => {}
            }
        }
    }
    for (i, t) in p.relation_types.iter().enumerate() {
        let path = format!("/relation_types/{i}");
        for (suffix, id) in [("source", &t.source), ("target", &t.target)] {
            if !types.contains_key(id.as_str()) {
                errors.add("REF_MISSING", &format!("{path}/{suffix}"), "endpoint type not found");
            }
        }
        if t.mode == Direction::Symmetric && t.source != t.target {
            errors.add("REL_SCHEMA", &path, "symmetric relation requires identical endpoint types");
        }
    }
    let mut pairs = BTreeSet::new();
    for (i, r) in p.relations.iter().enumerate() {
        if errors.full() {
            return errors.finish();
        }
        let path = format!("/relations/{i}");
        let Some(t) = relation_types.get(r.type_id.as_str()) else {
            errors.add("REF_MISSING", &format!("{path}/type"), "relation type not found");
            continue;
        };
        for (suffix, id, expected) in [
            ("source", &r.source, &t.source),
            ("target", &r.target, &t.target),
        ] {
            let ep = format!("{path}/{suffix}");
            match objects.get(id.as_str()) {
                None => errors.add("REF_MISSING", &ep, "endpoint object not found"),
                Some(o) if &o.type_id != expected => errors.add("REF_TYPE", &ep, "endpoint type mismatch"),
                _ => {}
            }
        }
        if !t.allow_self && r.source == r.target {
            errors.add("REL_SELF", &path, "self edge forbidden");
        }
        let (a, b) = if t.mode == Direction::Symmetric && r.source > r.target {
            errors.add("REL_ORDER", &path, "symmetric endpoints must be in ascending id order");
            (r.target.as_str(), r.source.as_str())
        } else {
            (r.source.as_str(), r.target.as_str())
        };
        if t.unique_pair && !pairs.insert((r.type_id.as_str(), a, b)) {
            errors.add("REL_DUPLICATE", &path, "duplicate endpoint pair");
        }
    }
    errors.finish()
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ReferenceEdge {
    pub from: String,
    pub kind: &'static str,
    pub field: Option<String>,
    pub to: String,
}

pub(crate) fn reference_edges(p: &Project) -> Vec<ReferenceEdge> {
    let mut edges = Vec::new();
    let mut push = |from: &str, kind, field: Option<&str>, to: &str| {
        edges.push(ReferenceEdge {
            from: from.into(),
            kind,
            field: field.map(str::to_owned),
            to: to.into(),
        });
    };
    for t in &p.types {
        for f in &t.fields {
            push(&t.id, "declares_field", None, &f.id);
            if let Some(target) = &f.target {
                push(&f.id, "reference_target", None, target);
            }
            for opt in &f.options {
                push(&f.id, "declares_option", None, &opt.id);
            }
        }
    }
    for o in &p.objects {
        push(&o.id, "object_type", None, &o.type_id);
        for (field, value) in &o.values {
            push(&o.id, "value_field", Some(field), field);
            match value {
                Value::Ref(id) => push(&o.id, "object_reference", Some(field), id),
                Value::Enum(id) => push(&o.id, "enum_selection", Some(field), id),
                _ => {}
            }
        }
    }
    for t in &p.relation_types {
        push(&t.id, "source_type", None, &t.source);
        push(&t.id, "target_type", None, &t.target);
    }
    for r in &p.relations {
        push(&r.id, "relation_type", None, &r.type_id);
        push(&r.id, "source_object", None, &r.source);
        push(&r.id, "target_object", None, &r.target);
    }
    edges.sort();
    edges
}
