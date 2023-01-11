docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i ./local/swagger.yaml \
    -g rust \
    -o /local \
    --global-property models,supportingFiles