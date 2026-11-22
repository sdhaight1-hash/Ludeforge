use crate::{validate, Diagnostic, Project, SAFE_INTEGER};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::fmt;

pub const MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;
const MAX_DEPTH: usize = 64;
const MAX_NODES: usize = 1_000_000;

// Deserialize before constructing typed records so duplicate keys cannot be collapsed.
struct Seed<'a> {
    depth: usize,
    remaining: &'a mut usize,
}

impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;

    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        if self.depth > MAX_DEPTH || *self.remaining == 0 {
            return Err(de::Error::custom("FORMAT_LIMIT: JSON nesting or node budget exceeded"));
        }
        *self.remaining -= 1;
        d.deserialize_any(StrictVisitor {
            depth: self.depth,
            remaining: self.remaining,
        })
    }
}

struct StrictVisitor<'a> {
    depth: usize,
    remaining: &'a mut usize,
}

impl<'de> Visitor<'de> for StrictVisitor<'_> {
    type Value = Value;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("v1 JSON without null, duplicate keys or noninteger numbers")
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(Value::String(v.to_owned()))
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Value, E> {
        Ok(Value::String(v))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Value, E> {
        if !(-SAFE_INTEGER..=SAFE_INTEGER).contains(&v) {
            return Err(E::custom("FORMAT_NUMBER: integer outside v1 safe domain"));
        }
        Ok(Value::from(v))
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Value, E> {
        if v > SAFE_INTEGER as u64 {
            return Err(E::custom("FORMAT_NUMBER: integer outside v1 safe domain"));
        }
        Ok(Value::from(v))
    }

    fn visit_f64<E: de::Error>(self, _: f64) -> Result<Value, E> {
        Err(E::custom("FORMAT_NUMBER: decimal and exponent numbers are not supported in v1"))
    }

    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
        Err(E::custom("FORMAT_NULL: null is not supported in v1"))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        while let Some(v) = seq.next_element_seed(Seed {
            depth: self.depth + 1,
            remaining: &mut *self.remaining,
        })? {
            values.push(v);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(de::Error::custom("FORMAT_DUPLICATE_KEY: duplicate JSON member"));
            }
            let value = map.next_value_seed(Seed {
                depth: self.depth + 1,
                remaining: &mut *self.remaining,
            })?;
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
}

fn parse_error(e: serde_json::Error) -> Vec<Diagnostic> {
    let message = e.to_string();
    let code = [
        "FORMAT_LIMIT",
        "FORMAT_NUMBER",
        "FORMAT_NULL",
        "FORMAT_DUPLICATE_KEY",
    ]
    .into_iter()
    .find(|code| message.starts_with(*code))
    .unwrap_or("FORMAT_JSON");
    vec![Diagnostic::new(code, "", message)]
}

/// Strict decoding and semantic validation. No partially valid Project is returned.
pub fn load(bytes: &[u8]) -> Result<Project, Vec<Diagnostic>> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(vec![Diagnostic::new("FORMAT_SIZE", "", "project exceeds 16 MiB")]);
    }
    let mut d = serde_json::Deserializer::from_slice(bytes);
    let mut remaining = MAX_NODES;
    let value = Seed {
        depth: 1,
        remaining: &mut remaining,
    }
    .deserialize(&mut d)
    .map_err(parse_error)?;
    d.end().map_err(parse_error)?;
    if value.get("format_version").and_then(Value::as_u64) != Some(1) {
        return Err(vec![Diagnostic::new(
            "FORMAT_VERSION",
            "/format_version",
            "expected format_version 1",
        )]);
    }
    let project: Project = serde_json::from_value(value)
        .map_err(|e| vec![Diagnostic::new("FORMAT_SHAPE", "", e.to_string())])?;
    validate(&project)?;
    Ok(project)
}
