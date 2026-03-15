DELETE {{host}}/


GET {{host}}/two-users

HTTP 403


POST {{host}}/
{}

HTTP 201
[Asserts]
jsonpath "$.id" exists
jsonpath "$.id" isNumber
jsonpath "$.id" == 1


GET {{host}}/two-users

HTTP 403


POST {{host}}/
{}

HTTP 201
[Asserts]
jsonpath "$.id" exists
jsonpath "$.id" isNumber
jsonpath "$.id" == 2


GET {{host}}/two-users

HTTP 200


POST {{host}}/
{}

HTTP 201
[Asserts]
jsonpath "$.id" exists
jsonpath "$.id" isNumber
jsonpath "$.id" == 3


GET {{host}}/two-users

HTTP 403


DELETE {{host}}/

HTTP 200
[Asserts]
jsonpath "$.version" == "0.1.0"
jsonpath "$.version" exists
jsonpath "$.version" isString
jsonpath "$.count" exists
jsonpath "$.count" isNumber


POST {{host}}/
{}

HTTP 201
[Asserts]
jsonpath "$.id" exists
jsonpath "$.id" isNumber
jsonpath "$.id" == 1
jsonpath "$.email" exists
jsonpath "$.email" isString
jsonpath "$.email" == "igor_1@example.com"
jsonpath "$.time" exists
jsonpath "$.time" isString
jsonpath "$.time" matches /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$/


POST {{host}}/
{}

HTTP 201
[Asserts]
jsonpath "$.id" exists
jsonpath "$.id" isNumber
jsonpath "$.id" == 2
jsonpath "$.email" exists
jsonpath "$.email" isString
jsonpath "$.email" == "igor_2@example.com"


GET {{host}}/

HTTP 200
[Asserts]
jsonpath "$.version" exists
jsonpath "$.version" isString
jsonpath "$.version" == "0.1.0"
jsonpath "$.count" exists
jsonpath "$.count" isNumber
jsonpath "$.count" == 2
jsonpath "$.users" exists
jsonpath "$.users" isCollection
jsonpath "$.users[0].id" == 1
jsonpath "$.users[1].id" == 2


GET {{host}}/healthz

HTTP 200
[Asserts]
jsonpath "$.version" exists
jsonpath "$.version" isString
jsonpath "$.version" == "0.1.0"
jsonpath "$.status" exists
jsonpath "$.status" isString
jsonpath "$.status" == "ok"
jsonpath "$.instance_id" exists
jsonpath "$.instance_id" isString
jsonpath "$.instance_id" matches /^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$/
jsonpath "$.time" exists
jsonpath "$.time" isString
jsonpath "$.time" matches /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$/
