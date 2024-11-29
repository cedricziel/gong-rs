## Regeneration

```bash
openapi-generator generate -g rust \
  -i spec/openapi.json \
  -o . \
  --additional-properties=useSingleRequestParameter=true,packageName=gong-rs,packageVersion=0.0.1 --skip-validate-spec
```
