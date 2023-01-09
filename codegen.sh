docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i ./local/swagger.yaml \
    -g rust \
    -o /local/codegen \
    --global-property models,supportingFiles