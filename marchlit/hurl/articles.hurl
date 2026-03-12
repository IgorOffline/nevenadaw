POST {{host}}/api/articles
{
  "article": {
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
