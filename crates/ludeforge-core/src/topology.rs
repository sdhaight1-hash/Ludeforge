//! Relation-kind-local cardinality and directed cycle checks.
use crate::{Direction, Project};
use crate::validation::Errors;
use std::collections::{BTreeMap, VecDeque};

pub(crate) fn check(p: &Project, errors: &mut Errors) {
    let mut grouped: BTreeMap<&str, Vec<(usize, &str, &str)>> = BTreeMap::new();
    for (i, r) in p.relations.iter().enumerate() {
        grouped.entry(&r.type_id).or_default().push((i, &r.source, &r.target));
    }
    for (i, t) in p.relation_types.iter().enumerate() {
        if errors.full() { return; }
        let path = format!("/relation_types/{i}");
        let symmetric = t.mode == Direction::Symmetric;
        if (symmetric && (t.max_in.is_some() || t.max_out.is_some() || t.acyclic)) || (!symmetric && t.max_degree.is_some()) {
            errors.add("REL_SCHEMA", &path, "directed relation uses max_in/max_out/acyclic; symmetric uses max_degree");
            continue;
        }
        let edges = grouped.get(t.id.as_str()).map(Vec::as_slice).unwrap_or_default();
        let mut incoming: BTreeMap<&str, usize> = BTreeMap::new();
        let mut outgoing: BTreeMap<&str, usize> = BTreeMap::new();
        let mut adjacency: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for &(j,a,b) in edges {
            if errors.full() { return; }
            let count = outgoing.entry(a).or_default();
            *count += 1;
            if symmetric {
                if t.max_degree.is_some_and(|max| *count > max as usize) {
                    errors.add("REL_CARDINALITY", &format!("/relations/{j}"), "source degree exceeds maximum");
                }
                if a != b {
                    let count = outgoing.entry(b).or_default();
                    *count += 1;
                    if t.max_degree.is_some_and(|max| *count > max as usize) {
                        errors.add("REL_CARDINALITY", &format!("/relations/{j}"), "target degree exceeds maximum");
                    }
                }
            } else {
                let n = incoming.entry(b).or_default();
                *n += 1;
                if t.max_in.is_some_and(|max| *n > max as usize) || t.max_out.is_some_and(|max| *count > max as usize) {
                    errors.add("REL_CARDINALITY", &format!("/relations/{j}"), "directed degree exceeds maximum");
                }
                if t.acyclic {
                    incoming.entry(a).or_default();
                    adjacency.entry(a).or_default().push(b);
                }
            }
        }
        if t.acyclic {
            let mut queue: VecDeque<_> = incoming.iter().filter(|(_, n)| **n == 0).map(|(&node, _)| node).collect();
            let mut visited = 0;
            while let Some(node) = queue.pop_front() {
                visited += 1;
                for target in adjacency.get(node).into_iter().flatten() {
                    let n = incoming.get_mut(target).expect("indexed endpoint");
                    *n -= 1;
                    if *n == 0 { queue.push_back(target); }
                }
            }
            if visited != incoming.len() { errors.add("REL_CYCLE", &path, "directed relation contains a cycle"); }
        }
    }
}
