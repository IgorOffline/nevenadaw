POST {{host}}/api/users
{
  "user": {
    "username": "art_{{uid}}",
    "email": "art_{{uid}}@test.com",
    "password": "password123" }
    }
HTTP 201
[Captures]
user_id: jsonpath "$.user.id"

POST {{host}}/api/articles
{
  "article": {
    "author_id": "{{user_id}}",
    "title": "Test Article {{uid}}",
    "description": "Test description",
    "body": "Test body content"
  }
}
HTTP 201
[Asserts]
jsonpath "$.article.title" == "Test Article {{uid}}"
jsonpath "$.article.description" == "Test description"
jsonpath "$.article.body" == "Test body content"

POST {{host}}/api/articles
{
  "article": {
    "author_id": "00000000-0000-0000-0000-000000000000",
    "title": "Bad Article",
    "description": "Bad",
    "body": "Bad"
  }
}
HTTP 422
[Asserts]
jsonpath "$.error" == "author_id must not be nil"

GET {{host}}/api/articles
HTTP 200
[Asserts]
jsonpath "$.articles" isList
jsonpath "$.articlesCount" isInteger
jsonpath "$.articlesCount" >= 1
jsonpath "$.articles[0].title" isString
jsonpath "$.articles[0].description" isString
jsonpath "$.articles[0].body" not exists

DELETE {{host}}/api/articles
HTTP 204
