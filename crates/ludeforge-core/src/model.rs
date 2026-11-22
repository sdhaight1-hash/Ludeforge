use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SAFE_INTEGER: i64 = 9_007_199_254_740_991;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub format_version: u32,
    pub project_id: String,
    pub label: String,
    pub types: Vec<EntityType>,
    pub objects: Vec<Object>,
    pub relation_types: Vec<RelationType>,
    pub relations: Vec<Relation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,
    /// Nonempty rule sets are rejected until the executable IR is implemented.
    pub rules: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityType {
    pub id: String,
    pub label: String,
    pub fields: Vec<Field>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Bool,
    Int,
    Text,
    Enum,
    Ref,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub id: String,
    pub label: String,
    pub kind: Kind,
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<EnumOption>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnumOption {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Text(String),
    Enum(String),
    Ref(String),
}

impl Value {
    pub fn kind(&self) -> Kind {
        match self {
            Self::Bool(_) => Kind::Bool,
            Self::Int(_) => Kind::Int,
            Self::Text(_) => Kind::Text,
            Self::Enum(_) => Kind::Enum,
            Self::Ref(_) => Kind::Ref,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub type_id: String,
    pub values: BTreeMap<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Directed,
    Symmetric,
}

fn yes() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationType {
    pub id: String,
    pub label: String,
    pub source: String,
    pub target: String,
    pub mode: Direction,
    #[serde(default)]
    pub unique_pair: bool,
    #[serde(default = "yes")]
    pub allow_self: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_out: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_in: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_degree: Option<u32>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub acyclic: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relation {
    pub id: String,
    #[serde(rename = "type")]
    pub type_id: String,
    pub source: String,
    pub target: String,
}

fn is_false(value: &bool) -> bool {
    !*value
}

/// Pure, bounded authored invariant. No randomness, effects, or runtime state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Constraint {
    Predicate { id: String, scope_type: String, expr: Expr },
    Unique { id: String, scope_type: String, fields: Vec<String> },
}

impl Constraint {
    pub fn id(&self) -> &str {
        match self {
            Self::Predicate { id, .. } | Self::Unique { id, .. } => id,
        }
    }

    pub fn scope(&self) -> &str {
        match self {
            Self::Predicate { scope_type, .. } | Self::Unique { scope_type, .. } => scope_type,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Expr {
    Int { value: i64 },
    Bool { value: bool },
    Read { field: String },
    RefRead { field: String, target_field: String },
    Present { field: String },
    Not { value: Box<Expr> },
    Add { left: Box<Expr>, right: Box<Expr> },
    Sub { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
    Le { left: Box<Expr>, right: Box<Expr> },
    Lt { left: Box<Expr>, right: Box<Expr> },
    Eq { left: Box<Expr>, right: Box<Expr> },
    And { left: Box<Expr>, right: Box<Expr> },
    Or { left: Box<Expr>, right: Box<Expr> },
}
