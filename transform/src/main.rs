use std::{env, fs};
use std::fs::File;
use serde_yaml::Value;
use openapiv3::OpenAPI;

// use as cargo run -- <input> <output>
fn main() {
    let mut args = env::args().skip(1);
    let fpath: String = args.next().expect("Pass in an input fpath to an openapi spec");
    let out_fpath: String = args.next().expect("Need an output fpath");

    let f = File::open(fpath).expect("Could not open file");
    let mut spec: Value = serde_yaml::from_reader(f).expect("Could not deserialize yaml from file");
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

    fs::write(&out_fpath, serde_yaml::to_string(&spec).unwrap()).expect("Could not write to file");
    eprintln!("Wrote to {}", out_fpath);
}
