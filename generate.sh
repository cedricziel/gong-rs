#!/bin/bash

openapi-generator generate -g rust \
  -i spec/openapi.json \
  -o . \
  --additional-properties=useSingleRequestParameter=true,packageName=gong-rs,packageVersion=0.0.1,infoEmail=mail@cedric-ziel.com,appDescription="A gong.io API client for the Rust language",licenseInfo="Apache-2.0",homePageUrl=https://github.com/cedricziel/gong-rs \
  --skip-validate-spec
