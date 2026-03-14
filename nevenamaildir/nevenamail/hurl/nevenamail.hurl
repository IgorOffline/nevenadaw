GET {{host}}/

HTTP 200
[Asserts]
jsonpath "$.id" exists
jsonpath "$.id" isInteger
jsonpath "$.id" >= 1
jsonpath "$.id" <= 6
jsonpath "$.time" exists
jsonpath "$.time" isString
jsonpath "$.time" matches /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$/


GET {{host}}/healthz

HTTP 200
[Asserts]
jsonpath "$.status" exists
jsonpath "$.status" isString
jsonpath "$.status" == "ok"
jsonpath "$.instance_id" exists
jsonpath "$.instance_id" isString
jsonpath "$.instance_id" matches /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/
jsonpath "$.time" exists
jsonpath "$.time" isString
jsonpath "$.time" matches /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$/
