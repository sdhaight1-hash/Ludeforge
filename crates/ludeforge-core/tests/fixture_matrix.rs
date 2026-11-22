use ludeforge_core::{load, normalize, references};
use serde_json::{json, Value};

const GLASS: &[u8] = include_bytes!("../../../examples/glass-market/project.json");
const TIDE: &[u8] = include_bytes!("../../../examples/tidemill-workshop/project.json");
const WAY: &[u8] = include_bytes!("../../../examples/wayfarer-chronicle/project.json");
const MOSS: &[u8] = include_bytes!("../../../examples/mosswater-crossing/project.json");

fn change(bytes: &[u8], edit: impl FnOnce(&mut Value)) -> Vec<u8> {
    let mut p: Value = serde_json::from_slice(bytes).unwrap();
    edit(&mut p);
    serde_json::to_vec(&p).unwrap()
}

fn reject(bytes: &[u8], code: &str) {
    let ds = load(bytes).expect_err("invalid project");
    assert!(ds.iter().any(|d| d.code == code), "{ds:?}");
}

#[test]
fn four_more_authored_philosophies() {
    for bytes in [GLASS, TIDE, WAY, MOSS] {
        let p = load(bytes).unwrap();
        let norm = normalize(&p).unwrap();
        let restored = load(&norm).unwrap();
        assert_eq!(normalize(&restored).unwrap(), norm);
        assert_eq!(references(&p).unwrap(), references(&restored).unwrap());
        assert!(!p.constraints.is_empty());
    }
}

#[test]
fn glass_budget_rejects_overspend_without_specialized_engine() {
    let invalid = change(GLASS, |p| {
        p["objects"][0]["values"]["training_cost"]["value"] = json!(5);
    });
    reject(&invalid, "CONSTRAINT_FALSE");
    let valid = change(GLASS, |p| {
        p["objects"][0]["values"]["equipment_cost"]["value"] = json!(4);
        p["objects"][0]["values"]["contact_cost"]["value"] = json!(1);
    });
    assert!(load(&valid).is_ok());
}

#[test]
fn tidemill_initial_and_completed_snapshots_conserve_material() {
    let completed = change(TIDE, |p| {
        let v = &mut p["objects"][0]["values"];
        v["reserved"]["value"] = json!(0);
        v["output"]["value"] = json!(3);
        v["waste"]["value"] = json!(1);
        v["operations_left"]["value"] = json!(0);
    });
    assert!(load(&completed).is_ok());
    let duplicated = change(&completed, |p| {
        p["objects"][0]["values"]["output"]["value"] = json!(6);
    });
    reject(&duplicated, "CONSTRAINT_FALSE");
}

#[test]
fn wayfarer_graph_and_progress_limits() {
    let shortened = change(WAY, |p| {
        p["objects"][0]["values"]["capacity"]["value"] = json!(0);
    });
    reject(&shortened, "CONSTRAINT_FALSE");
    let cyclic = change(WAY, |p| {
        p["relations"].as_array_mut().unwrap().push(json!({
            "id":"reverse_stage","type":"next_stage","source":"later","target":"early"
        }));
    });
    reject(&cyclic, "REL_CYCLE");
}

#[test]
fn mosswater_injury_refers_to_typed_body_part() {
    let bad_severity = change(MOSS, |p| {
        p["objects"][2]["values"]["severity"]["value"] = json!(6);
    });
    reject(&bad_severity, "CONSTRAINT_FALSE");
    let wrong_target = change(MOSS, |p| {
        p["objects"][2]["values"]["affected_limb"]["value"] = json!("scout");
    });
    reject(&wrong_target, "REF_TYPE");
}

#[test]
fn global_constraint_budget_fails_closed() {
    let mut p: Value = serde_json::from_slice(GLASS).unwrap();
    p["types"][0]["fields"] = json!([]);
    p["objects"] = json!((0..1001)
        .map(|i| json!({"id":format!("row{i}"),"type":"allocation","label":"Row","values":{}}))
        .collect::<Vec<_>>());
    p["constraints"] = json!((0..1000)
        .map(|i| json!({"id":format!("predicate{i}"),"kind":"predicate","scope_type":"allocation","expr":{"op":"bool","value":true}}))
        .collect::<Vec<_>>());
    reject(&serde_json::to_vec(&p).unwrap(), "CONSTRAINT_LIMIT");
}
