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
