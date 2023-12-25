transform:
    test -e openapi.yaml || req https://raw.githubusercontent.com/Asana/openapi/master/defs/asana_oas.yaml > openapi.yaml
    cd transform && cargo run -- ../openapi.yaml ../transform.yaml

generate:
    checkexec transform.yaml -- just transform
    libninja gen --verbose -lrust asana transform.yaml

meta:
    checkexec transform.yaml -- just transform
    export FOO=$PWD && cd ~/work/libninja/libninja && cargo run -- meta -lrust  --verbose asana $FOO/transform.yaml -o $FOO

generate_dev:
    checkexec transform.yaml -- just transform
    export FOO=$PWD && cd ~/work/libninja/libninja && cargo run -- gen -lrust  --verbose asana $FOO/transform.yaml -o $FOO

test:
    cargo test
