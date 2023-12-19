use std::{env, fs};
use std::fs::File;
use serde_yaml::Value;
use openapiv3::OpenAPI;


pub trait YamlValueExt {
    fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Value>;
}


fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

impl YamlValueExt for Value {
    fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Value> {
        if pointer.is_empty() {
            return Some(self);
        }
        pointer
            .split('/')
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self, |target, token| {
                match target {
                    Value::Mapping(map) => map.get_mut(&token),
                    Value::Sequence(list) => parse_index(&token).and_then(move |x| list.get_mut(x)),
                    _ => None,
                }
            })
    }
}

// use as cargo run -- <input> <output>
fn main() {
    let mut args = env::args().skip(1);
    let fpath: String = args.next().expect("Pass in an input fpath to an openapi spec");
    let out_fpath: String = args.next().expect("Need an output fpath");

    let f = File::open(fpath).expect("Could not open file");
    let mut spec: Value = serde_yaml::from_reader(f).expect("Could not deserialize yaml from file");

    // rename the schemas that encounter naming conflicts for libninja gen.
    // technically this should be handled by libninja itself, so this is working around a bug in libninja.
    let schema = spec["components"]["schemas"].as_mapping_mut().unwrap().remove("CreateMembershipRequest").unwrap();
    spec["components"]["schemas"]["CreateMembershipRequestBody"] = schema;
    let body = &mut spec["paths"]["/memberships"]["post"]["requestBody"]["content"]["application/json"];
    body["schema"]["properties"]["data"]["$ref"] = Value::String("#/components/schemas/CreateMembershipRequestBody".to_string());

    let schema = spec["components"]["schemas"].as_mapping_mut().unwrap().remove("ProjectSaveAsTemplateRequest").unwrap();
    spec["components"]["schemas"]["ProjectSaveAsTemplateRequestBody"] = schema;
    let body = &mut spec["paths"]["/projects/{project_gid}/saveAsTemplate"]["post"]["requestBody"]["content"]["application/json"];
    body["schema"]["properties"]["data"]["$ref"] = Value::String("#/components/schemas/ProjectSaveAsTemplateRequestBody".to_string());

    let schema = spec["components"]["schemas"].as_mapping_mut().unwrap().remove("CreateTimeTrackingEntryRequest").unwrap();
    spec["components"]["schemas"]["CreateTimeTrackingEntryRequestBody"] = schema;
    let body = &mut spec["paths"]["/tasks/{task_gid}/time_tracking_entries"]["post"]["requestBody"]["content"]["application/json"];
    body["schema"]["properties"]["data"]["$ref"] = Value::String("#/components/schemas/CreateTimeTrackingEntryRequestBody".to_string());

    let schema = spec["components"]["schemas"].as_mapping_mut().unwrap().remove("UpdateTimeTrackingEntryRequest").unwrap();
    spec["components"]["schemas"]["UpdateTimeTrackingEntryRequestBody"] = schema;
    let body = &mut spec["paths"]["/time_tracking_entries/{time_tracking_entry_gid}"]["put"]["requestBody"]["content"]["application/json"];
    body["schema"]["properties"]["data"]["$ref"] = Value::String("#/components/schemas/UpdateTimeTrackingEntryRequestBody".to_string());

    spec["paths"]["/users"]["get"]["responses"][200].as_mapping_mut().unwrap().remove("$ref").expect("Remove wrong ref. gj asana.");

    // TODO mark most fields as required. seems like the spec doesn't do that...
    'outer: for (path, item) in spec["paths"].as_mapping_mut().unwrap() {
        for (method, op) in item.as_mapping_mut().unwrap() {
            let Some(req) = op.pointer_mut("requestBody/content/application~1json/schema") else { continue; };
            if req["type"] == "object" {
                let keys = req["properties"].as_mapping().unwrap().keys().cloned().collect::<Vec<_>>();
                req["required"] = keys.into();
            }

            let Some(res) = op.get_mut("responses") else { continue; };
            for (code, res) in res.as_mapping_mut().unwrap() {
                let Some(res) = res.pointer_mut("content/application~1json/schema") else { continue; };
                if res["type"] == "object" {
                    let keys = res["properties"].as_mapping().unwrap().keys().cloned().collect::<Vec<_>>();
                    res["required"] = keys.into();
                }
            }
        }
    }
    fs::write(&out_fpath, serde_yaml::to_string(&spec).unwrap()).expect("Could not write to file");
    eprintln!("Wrote to {}", out_fpath);
}
