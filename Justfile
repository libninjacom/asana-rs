transform:
    req https://raw.githubusercontent.com/Asana/openapi/master/defs/asana_oas.yaml > openapi.yaml
    cd transform && cargo run -- ../openapi.yaml ../transform.yaml

generate:
    checkexec transform.yaml -- just transform
    libninja gen --verbose -lrust asana transform.yaml

test:
    cargo test