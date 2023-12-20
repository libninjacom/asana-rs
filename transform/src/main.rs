use std::{env, fs};
use std::fs::File;
use serde_yaml::Value;
use serde_yaml_ext::ValueExt;

// use as cargo run -- <input> <output>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let fpath: String = args.next().expect("Need an input fpath to an openapi spec");
    let out_fpath: String = args.next().expect("Need an output fpath");

    let f = File::open(fpath)?;
    let mut spec: Value = serde_yaml::from_reader(f)?;

    let rename_schema = |name: &str, path: &str, method: &str| {
        let schemas = spec.path_mut("components.schemas");
        let schema = schemas.remove(name).unwrap();
        let new_name = format!("{}Body", name);
        schemas.insert(&new_name, schema);
        let path = format!("paths.{path}.{method}.requestBody.content.application/json.schema.properties.data");
        spec.path_mut(&path).insert("$ref", format!("#/components/schemas/{new_name}"));
    };
    // rename the schemas that encounter naming conflicts for libninja gen.
    // technically this should be handled by libninja itself, so this is working around a bug in libninja.
    rename_schema("CreateMembershipRequest", "/memberships", "post");
    rename_schema("ProjectSaveAsTemplateRequest", "/projects/{project_gid}/saveAsTemplate", "post");
    rename_schema("CreateTimeTrackingEntryRequest", "/tasks/{task_gid}/time_tracking_entries", "post");
    rename_schema("UpdateTimeTrackingEntryRequest", "/time_tracking_entries/{time_tracking_entry_gid}", "put");

    spec.path_mut("paths./users.get.responses.200").remove("$ref").expect("Remove wrong ref. gj asana.");

    // mark most fields in operations as required.
    'outer: for item in spec["paths"].iter_mut() {
        for op in item.iter_mut() {
            let Some(req) = op.get_path_mut("requestBody.content.application/json.schema") else { continue; };
            if req.path("type") == "object" {
                let keys = req.path("properties").keys().cloned().collect::<Vec<_>>();
                res.insert("required", keys);
            }

            let Some(res) = op.get_mut("responses") else { continue; };
            for res in res.iter_mut() {
                let Some(res) = res.get_path_mut("content.application/json.schema") else { continue; };
                if res.path("type") == "object" {
                    let keys = res.path("properties").keys().cloned().collect::<Vec<_>>();
                    res.insert("required", keys);
                }
            }
        }
    }

    // do the same for schemas
    for schema in spec.path_mut("components.schemas").iter_mut() {
        if schema.path("type") == "object" {
            let Some(props) = schema.get_mut("properties") else { continue; };
            let keys = props.keys().cloned().collect::<Vec<_>>();
            schema.insert("required", keys);
        }
    }

    fs::write(&out_fpath, serde_yaml::to_string(&spec).unwrap()).expect("Could not write to file");
    eprintln!("Wrote to {}", out_fpath);
    Ok(())
}
