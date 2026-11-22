//! The first pure, typed expression subset. Compile against schema before reading instances.
use crate::{Constraint, Expr, Field, Kind, Object, Project, Value, SAFE_INTEGER};
use crate::validation::Errors;
use std::collections::{BTreeMap, BTreeSet};

type Fields<'a> = BTreeMap<&'a str, BTreeMap<&'a str, &'a Field>>;
type Objects<'a> = BTreeMap<&'a str, &'a Object>;
const MAX_EXPR_DEPTH: usize = 32;
const MAX_EXPR_NODES: usize = 4096;
const MAX_VALIDATION_STEPS: usize = 1_000_000;

fn spend(budget: &mut usize) -> Result<(), &'static str> {
    *budget = budget.checked_sub(1).ok_or("constraint step budget exceeded")?;
    Ok(())
}

fn schema_field<'a>(fields: &Fields<'a>, scope: &str, field: &str) -> Result<&'a Field, &'static str> {
    fields.get(scope).and_then(|f| f.get(field)).copied().ok_or("field not declared on scope")
}

fn scalar(kind: Kind) -> Result<Kind, &'static str> {
    match kind {
        Kind::Int | Kind::Bool => Ok(kind),
        _ => Err("expression subset supports integer and boolean operands only"),
    }
}

fn infer(e: &Expr, scope: &str, fields: &Fields<'_>, depth: usize, nodes: &mut usize, total: &mut usize) -> Result<Kind, &'static str> {
    if depth > MAX_EXPR_DEPTH { return Err("expression nesting limit exceeded"); }
    spend(nodes)?;
    spend(total)?;
    match e {
        Expr::Int { value } if (-SAFE_INTEGER..=SAFE_INTEGER).contains(value) => Ok(Kind::Int),
        Expr::Int { .. } => Err("integer literal outside safe domain"),
        Expr::Bool { .. } => Ok(Kind::Bool),
        Expr::Read { field } => scalar(schema_field(fields, scope, field)?.kind),
        Expr::Present { field } => {
            schema_field(fields, scope, field)?;
            Ok(Kind::Bool)
        }
        Expr::RefRead { field, target_field } => {
            let reference = schema_field(fields, scope, field)?;
            let target = reference.target.as_deref().ok_or("ref_read requires a reference field")?;
            scalar(schema_field(fields, target, target_field)?.kind)
        }
        Expr::Not { value } => {
            if infer(value, scope, fields, depth + 1, nodes, total)? != Kind::Bool {
                return Err("not requires boolean");
            }
            Ok(Kind::Bool)
        }
        Expr::Add { left, right } | Expr::Sub { left, right } | Expr::Mul { left, right }
        | Expr::Le { left, right } | Expr::Lt { left, right } | Expr::Eq { left, right }
        | Expr::And { left, right } | Expr::Or { left, right } => {
            let a = infer(left, scope, fields, depth + 1, nodes, total)?;
            let b = infer(right, scope, fields, depth + 1, nodes, total)?;
            match e {
                Expr::Eq { .. } if a == b => Ok(Kind::Bool),
                Expr::And { .. } | Expr::Or { .. } if a == Kind::Bool && b == a => Ok(Kind::Bool),
                Expr::Le { .. } | Expr::Lt { .. } if a == Kind::Int && b == a => Ok(Kind::Bool),
                Expr::Add { .. } | Expr::Sub { .. } | Expr::Mul { .. } if a == Kind::Int && b == a => Ok(Kind::Int),
                _ => Err("incompatible expression operands"),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scalar { Int(i64), Bool(bool) }

fn read(o: &Object, field: &str) -> Result<Scalar, &'static str> {
    match o.values.get(field) {
        Some(Value::Int(v)) => Ok(Scalar::Int(*v)),
        Some(Value::Bool(v)) => Ok(Scalar::Bool(*v)),
        _ => Err("missing or incompatible operand"),
    }
}

fn integer(v: Scalar) -> Result<i64, &'static str> {
    match v { Scalar::Int(i) => Ok(i), _ => Err("integer required") }
}

fn boolean(v: Scalar) -> Result<bool, &'static str> {
    match v { Scalar::Bool(b) => Ok(b), _ => Err("boolean required") }
}

fn arithmetic(v: Option<i64>) -> Result<Scalar, &'static str> {
    v.filter(|n| (-SAFE_INTEGER..=SAFE_INTEGER).contains(n))
        .map(Scalar::Int).ok_or("integer overflow")
}

fn eval(e: &Expr, o: &Object, objects: &Objects<'_>, budget: &mut usize) -> Result<Scalar, &'static str> {
    spend(budget)?;
    match e {
        Expr::Int { value } => Ok(Scalar::Int(*value)),
        Expr::Bool { value } => Ok(Scalar::Bool(*value)),
        Expr::Read { field } => read(o, field),
        Expr::Present { field } => Ok(Scalar::Bool(o.values.contains_key(field))),
        Expr::RefRead { field, target_field } => {
            let Some(Value::Ref(id)) = o.values.get(field) else { return Err("missing reference operand"); };
            let target = objects.get(id.as_str()).ok_or("missing reference object")?;
            read(target, target_field)
        }
        Expr::Not { value } => Ok(Scalar::Bool(!boolean(eval(value, o, objects, budget)?)?)),
        Expr::And { left, right } => {
            let a = boolean(eval(left, o, objects, budget)?)?;
            Ok(Scalar::Bool(a && boolean(eval(right, o, objects, budget)?)?))
        }
        Expr::Or { left, right } => {
            let a = boolean(eval(left, o, objects, budget)?)?;
            Ok(Scalar::Bool(a || boolean(eval(right, o, objects, budget)?)?))
        }
        Expr::Eq { left, right } => Ok(Scalar::Bool(eval(left, o, objects, budget)? == eval(right, o, objects, budget)?)),
        Expr::Add { left, right } | Expr::Sub { left, right } | Expr::Mul { left, right }
        | Expr::Le { left, right } | Expr::Lt { left, right } => {
            let a = integer(eval(left, o, objects, budget)?)?;
            let b = integer(eval(right, o, objects, budget)?)?;
            match e {
                Expr::Add { .. } => arithmetic(a.checked_add(b)),
                Expr::Sub { .. } => arithmetic(a.checked_sub(b)),
                Expr::Mul { .. } => arithmetic(a.checked_mul(b)),
                Expr::Le { .. } => Ok(Scalar::Bool(a <= b)),
                Expr::Lt { .. } => Ok(Scalar::Bool(a < b)),
                _ => unreachable!("matched numeric operations"),
            }
        }
    }
}

/// Run after local/reference validation. Compile every constraint even for empty scopes.
pub(crate) fn check(p: &Project, fields: &Fields<'_>, objects: &Objects<'_>, errors: &mut Errors) {
    let mut by_type: BTreeMap<&str, Vec<(usize, &Object)>> = BTreeMap::new();
    for (i, o) in p.objects.iter().enumerate() {
        by_type.entry(&o.type_id).or_default().push((i, o));
    }
    let mut budget = MAX_VALIDATION_STEPS;
    for (i, c) in p.constraints.iter().enumerate() {
        if errors.full() { return; }
        let path = format!("/constraints/{i}");
        if !fields.contains_key(c.scope()) {
            errors.add("CONSTRAINT_SCHEMA", &path, "unknown scope type");
            continue;
        }
        let rows = by_type.get(c.scope()).map(Vec::as_slice).unwrap_or_default();
        match c {
            Constraint::Predicate { expr, .. } => {
                let mut nodes = MAX_EXPR_NODES;
                match infer(expr, c.scope(), fields, 1, &mut nodes, &mut budget) {
                    Ok(Kind::Bool) => {}
                    Ok(_) => { errors.add("CONSTRAINT_SCHEMA", &path, "predicate must return boolean"); continue; }
                    Err(message) => { errors.add("CONSTRAINT_SCHEMA", &path, message); if budget == 0 { return; } continue; }
                }
                for (j, o) in rows {
                    if errors.full() { return; }
                    let result = eval(expr, o, objects, &mut budget);
                    let op = format!("/objects/{j}");
                    match result {
                        Ok(Scalar::Bool(true)) => {}
                        Ok(Scalar::Bool(false)) => errors.add("CONSTRAINT_FALSE", &op, format!("{} violated on {}", c.id(), o.id)),
                        _ if budget == 0 => { errors.add("CONSTRAINT_LIMIT", &path, "global validation step budget exceeded"); return; }
                        Err(message) => errors.add("CONSTRAINT_EVALUATION", &op, format!("{} on {}: {message}", c.id(), o.id)),
                        Ok(_) => errors.add("CONSTRAINT_EVALUATION", &op, "unexpected result type"),
                    }
                }
            }
            Constraint::Unique { fields: keys, .. } => {
                let distinct: BTreeSet<_> = keys.iter().collect();
                if keys.is_empty() || keys.len() > 32 || distinct.len() != keys.len() || keys.iter().any(|k| !matches!(schema_field(fields,c.scope(),k), Ok(f) if f.required)) {
                    errors.add("CONSTRAINT_SCHEMA", &path, "unique key requires 1–32 distinct required fields on scope");
                    continue;
                }
                let mut seen = BTreeMap::<Vec<&Value>, &str>::new();
                for (j, o) in rows {
                    if errors.full() { return; }
                    if budget < keys.len() { errors.add("CONSTRAINT_LIMIT", &path, "global validation step budget exceeded"); return; }
                    budget -= keys.len();
                    let Some(tuple) = keys.iter().map(|k| o.values.get(k)).collect::<Option<Vec<_>>>() else {
                        errors.add("CONSTRAINT_EVALUATION", &format!("/objects/{j}"), "missing unique key field");
                        continue;
                    };
                    if let Some(first) = seen.get(&tuple) {
                        errors.add("UNIQUE_KEY", &format!("/objects/{j}"), format!("{}: {} conflicts with {first}", c.id(), o.id));
                    } else {
                        seen.insert(tuple, &o.id);
                    }
                }
            }
        }
    }
}

/// Static possible reads, including both short-circuit branches. Called only after validation.
pub(crate) fn reads(c: &Constraint) -> BTreeSet<&str> {
    fn visit<'a>(expr: &'a Expr, out: &mut BTreeSet<&'a str>) {
        match expr {
            Expr::Read { field } | Expr::Present { field } => { out.insert(field); }
            Expr::RefRead { field, target_field } => { out.insert(field); out.insert(target_field); }
            Expr::Not { value } => visit(value, out),
            Expr::Add { left,right } | Expr::Sub { left,right } | Expr::Mul { left,right } |
            Expr::Le { left,right } | Expr::Lt { left,right } | Expr::Eq { left,right } |
            Expr::And { left,right } | Expr::Or { left,right } => { visit(left,out); visit(right,out); }
            Expr::Int { .. } | Expr::Bool { .. } => {}
        }
    }
    let mut result = BTreeSet::new();
    match c {
        Constraint::Predicate { expr, .. } => visit(expr, &mut result),
        Constraint::Unique { fields, .. } => result.extend(fields.iter().map(String::as_str)),
    }
    result
}
