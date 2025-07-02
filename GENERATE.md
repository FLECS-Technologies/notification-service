# How to generate the notis_server code

We use the [rust-axum generator](https://openapi-generator.tech/docs/generators/rust-axum)
from [openapi-generator.tech](https://openapi-generator.tech/) to generate the code for the notis_server.

## Generate the code

Execute the following command from the repository root directory. Adjust the version if necessary
``docker run --rm -v ${PWD}:/local --user $(id -u):$(id -g) openapitools/openapi-generator-cli:v7.11.0 generate -i /local/api/openapi.yaml --additional-properties=packageName=notis_server,generateAliasAsModel=true,packageVersion=0.1.0 -g rust-axum -o /local/notis_server && cargo fmt``

## Format the code

Format the generated code using [rustfmt](https://github.com/rust-lang/rustfmt): ``cargo fmt``

## Manual adjustments

We currently need access to `notis_server::types::Object.0` which can not be accessed in the generated code. We
therefore made it accessible in `notis_server/src/types.rs` and excluded the file from generating again in
`notis_server/.openapi-generator-ignore`.