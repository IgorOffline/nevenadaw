GET {{host}}/api/version
HTTP 200
[Asserts]
jsonpath "$.versions" count == 1
jsonpath "$.versions[0]" == "0.1.0"
