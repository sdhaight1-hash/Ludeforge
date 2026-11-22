use ludeforge_core::{load, normalize, references, validate, Direction, Project, Value, MAX_INPUT_BYTES, SAFE_INTEGER};
use serde_json::{json, Value as Json};

const LANTERN: &[u8] = include_bytes!("../../../examples/lantern-academy/project.json");
const COUNCIL: &[u8] = include_bytes!("../../../examples/council-of-reeds/project.json");

fn fixture(bytes: &[u8]) -> Json {
    serde_json::from_slice(bytes).expect("fixture JSON")
}

fn reject(data: &[u8], code: &str) {
    let ds = load(data).expect_err("must reject invalid project");
    assert!(ds.iter().any(|d| d.code == code), "expected {code}: {ds:?}");
}

fn mutated(bytes: &[u8], change: impl FnOnce(&mut Json), code: &str) {
    let mut value = fixture(bytes);
    change(&mut value);
    reject(&serde_json::to_vec(&value).unwrap(), code);
}

#[test]
fn divergent_projects_and_empty_ontology() {
    for bytes in [LANTERN, COUNCIL] {
        let p = load(bytes).unwrap();
        assert!(validate(&p).is_ok());
        assert!(p.rules.is_empty());
    }
    let empty = br#"{"format_version":1,"project_id":"empty","label":"Empty","types":[],"objects":[],"relation_types":[],"relations":[],"rules":[]}"#;
    let p = load(empty).unwrap();
    assert!(references(&p).unwrap().is_empty());
    assert_eq!(load(&normalize(&p).unwrap()).unwrap(), p);
}

#[test]
fn normalized_round_trip_reordering_and_unicode() {
    for bytes in [LANTERN, COUNCIL] {
        let mut p = load(bytes).unwrap();
        p.label = "Reeds — Café 雪".into();
        let first = normalize(&p).unwrap();
        let canonical = load(&first).unwrap();
        assert_eq!(normalize(&canonical).unwrap(), first);
        p.types.reverse();
        p.objects.reverse();
        p.relations.reverse();
        p.relation_types.reverse();
        for t in &mut p.types {
            t.fields.reverse();
            for f in &mut t.fields {
                f.options.reverse();
            }
        }
        assert_eq!(normalize(&p).unwrap(), first);
        assert_eq!(references(&canonical).unwrap(), references(&p).unwrap());
    }
}

#[test]
fn duplicate_keys_at_every_level_and_escaped_alias() {
    let raw = String::from_utf8(LANTERN.to_vec()).unwrap();
    for text in [
        raw.replace("\"format_version\": 1", "\"format_version\": 1, \"format_version\": 1"),
        raw.replace("\"focus\": {", "\"focus\": {\"kind\":\"int\",\"value\":0}, \"focus\": {"),
        raw.replace("\"value\": 3", "\"value\": 3, \"\\u0076alue\": 3"),
    ] {
        reject(text.as_bytes(), "FORMAT_DUPLICATE_KEY");
    }
}

#[test]
fn strict_raw_json_and_limits() {
    let raw = String::from_utf8(LANTERN.to_vec()).unwrap();
    for text in [
        raw.replace("\"value\": 3", "\"value\": 3.0"),
        raw.replace("\"value\": 3", "\"value\": 3e0"),
        raw.replace("\"value\": 3", "\"value\": 9007199254740992"),
        raw.replace("\"value\": 3", "\"value\": -9007199254740992"),
    ] {
        reject(text.as_bytes(), "FORMAT_NUMBER");
    }
    reject(raw.replace("\"value\": 3", "\"value\": null").as_bytes(), "FORMAT_NULL");
    reject(format!("{raw} null").as_bytes(), "FORMAT_JSON");
    reject(b"\xff", "FORMAT_JSON");
    reject(&vec![b' '; MAX_INPUT_BYTES + 1], "FORMAT_SIZE");
    let deep = format!("{}0{}", "[".repeat(67), "]".repeat(67));
    reject(deep.as_bytes(), "FORMAT_LIMIT");
}

#[test]
fn structural_failures_and_unsupported_rules() {
    mutated(LANTERN, |p| p["format_version"] = json!(2), "FORMAT_VERSION");
    mutated(LANTERN, |p| p["surprise"] = json!(true), "FORMAT_SHAPE");
    mutated(LANTERN, |p| p["types"][0]["fields"][0]["surprise"] = json!(true), "FORMAT_SHAPE");
    mutated(LANTERN, |p| p["objects"][0]["values"]["max_rank"]["extra"] = json!(true), "FORMAT_SHAPE");
    mutated(LANTERN, |p| p["rules"] = json!([{"id":"future"}]), "RULE_UNSUPPORTED");
    mutated(LANTERN, |p| p["types"][0]["fields"][0]["min"] = json!(8), "FIELD_SCHEMA");
    mutated(LANTERN, |p| p["types"][1]["fields"][0]["target"] = json!("no_type"), "REF_MISSING");
}

#[test]
fn identity_field_and_reference_rejections() {
    mutated(LANTERN, |p| p["objects"][0]["id"] = json!("role"), "ID_DUPLICATE");
    mutated(LANTERN, |p| p["objects"][0]["id"] = json!("9bad"), "ID_INVALID");
    mutated(LANTERN, |p| p["objects"][1]["type"] = json!("unknown"), "REF_MISSING");
    mutated(LANTERN, |p| {p["objects"][1]["values"].as_object_mut().unwrap().remove("focus");}, "FIELD_REQUIRED");
    mutated(LANTERN, |p| p["objects"][1]["values"]["new"] = json!({"kind":"int","value":1}), "FIELD_UNKNOWN");
    mutated(LANTERN, |p| p["objects"][1]["values"]["rank"] = json!({"kind":"text","value":"2"}), "VALUE_KIND");
    mutated(LANTERN, |p| p["objects"][1]["values"]["rank"]["value"] = json!(4), "VALUE_RANGE");
    mutated(LANTERN, |p| p["objects"][1]["values"]["role_ref"]["value"] = json!("missing"), "REF_MISSING");
    mutated(LANTERN, |p| p["objects"][1]["values"]["role_ref"]["value"] = json!("first_trial"), "REF_TYPE");
    mutated(COUNCIL, |p| p["objects"][2]["values"]["status"]["value"] = json!("absent"), "ENUM_OPTION");
}

#[test]
fn optional_absence_boolean_text_and_safe_integer_domain() {
    let mut data = fixture(LANTERN);
    let fields = data["types"][1]["fields"].as_array_mut().unwrap();
    fields.push(json!({"id":"note","label":"Note","kind":"text","required":false}));
    fields.push(json!({"id":"active","label":"Active","kind":"bool","required":true}));
    fields.push(json!({"id":"wide","label":"Wide","kind":"int","required":true,"min":-SAFE_INTEGER,"max":SAFE_INTEGER}));
    data["objects"][1]["values"]["active"] = json!({"kind":"bool","value":false});
    data["objects"][1]["values"]["wide"] = json!({"kind":"int","value":SAFE_INTEGER});
    let p = load(&serde_json::to_vec(&data).unwrap()).unwrap();
    let recruit = p.objects.iter().find(|o| o.id == "neri").unwrap();
    assert!(!recruit.values.contains_key("note"));
    assert_eq!(recruit.values["active"], Value::Bool(false));
    assert_eq!(recruit.values["wide"], Value::Int(SAFE_INTEGER));
}

#[test]
fn relationship_endpoints_duplicates_and_direction() {
    mutated(COUNCIL, |p| p["relations"][0]["source"] = json!("bridge"), "REF_TYPE");
    mutated(COUNCIL, |p| p["relations"][0]["source"] = json!("missing"), "REF_MISSING");
    mutated(COUNCIL, |p| {
        let mut edge = p["relations"][0].clone();
        edge["id"] = json!("second_edge");
        p["relations"].as_array_mut().unwrap().push(edge);
    }, "REL_DUPLICATE");
    mutated(COUNCIL, |p| p["relation_types"][0]["mode"] = json!("symmetric"), "REL_SCHEMA");
    let mut p = load(COUNCIL).unwrap();
    p.relation_types[0].target = "delegate".into();
    p.relation_types[0].mode = Direction::Symmetric;
    p.relation_types[0].allow_self = false;
    p.relations.truncate(1);
    p.relations[0].source = "moss".into();
    p.relations[0].target = "reed".into();
    assert!(validate(&p).is_ok());
    p.relations[0].source = "reed".into();
    p.relations[0].target = "moss".into();
    assert!(validate(&p).unwrap_err().iter().any(|d| d.code == "REL_ORDER"));
    p.relations[0].target = "reed".into();
    assert!(validate(&p).unwrap_err().iter().any(|d| d.code == "REL_SELF"));
}

#[test]
fn reference_index_and_rename_preserve_identity() {
    let mut p = load(LANTERN).unwrap();
    let edges = references(&p).unwrap();
    assert!(edges.iter().any(|e| e.from == "neri" && e.kind == "object_reference" && e.field.as_deref() == Some("role_ref") && e.to == "lantern_role"));
    p.types[0].label = "Profession".into();
    p.objects[0].label = "Renamed role".into();
    assert_eq!(references(&p).unwrap(), edges);
    p.objects[0].id = "different".into();
    assert!(references(&p).is_err());
    assert!(normalize(&p).is_err());
}

#[test]
fn public_validation_rejects_out_of_domain_data_and_caps_diagnostics() {
    let mut p: Project = load(LANTERN).unwrap();
    p.objects[1].values.insert("rank".into(), Value::Int(i64::MAX));
    assert!(validate(&p).unwrap_err().iter().any(|d| d.code == "VALUE_RANGE"));
    for i in 0..300 {
        p.objects[1].values.insert(format!("unknown{i}"), Value::Bool(false));
    }
    let ds = validate(&p).unwrap_err();
    assert!(ds.len() <= 257);
    assert!(ds.iter().any(|d| d.code == "DIAGNOSTIC_LIMIT"));
    assert!(ds.windows(2).all(|w| w[0] <= w[1]));
}
