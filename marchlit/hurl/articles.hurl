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

POST {{host}}/api/users
{
  "user": {
    "username": "art_mentor",
    "email": "art_mentor@test.com",
    "password": "password123" }
    }
HTTP 201
[Captures]
mentor_id: jsonpath "$.user.id"

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
[Captures]
article_id: jsonpath "$.article.id"

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

POST {{host}}/api/articles/{{article_id}}/comments
{
  "comment": {
    "body": "Test comment body"
  }
}
HTTP 201
[Asserts]
jsonpath "$.comment.id" isInteger
jsonpath "$.comment.author.username" == "cmt_{{uid}}"
[Captures]
comment_id: jsonpath "$.comment.id"

GET {{host}}/api/articles/{{article_id}}/comments
HTTP 200
[Asserts]
jsonpath "$.comments" isList
jsonpath "$.comments" count == 1
jsonpath "$.comments[0].id" == {{comment_id}}
jsonpath "$.comments[0].body" == "Test comment body"
jsonpath "$.comments[0].author.username" == "cmt_{{uid}}"

DELETE {{host}}/api/articles/{{article_id}}/comments
HTTP 204

POST {{host}}/api/articles/{{article_id}}/favorite
HTTP 200
[Asserts]
jsonpath "$.article.title" isString
jsonpath "$.article.description" isString
jsonpath "$.article.body" isString
jsonpath "$.article.favorited" == true
jsonpath "$.article.favoritesCount" == 1
jsonpath "$.article.author.username" == "fav_{{uid}}"

DELETE {{host}}/api/articles/{{article_id}}/favorite
HTTP 200
[Asserts]
jsonpath "$.article.title" isString
jsonpath "$.article.description" isString
jsonpath "$.article.body" isString
jsonpath "$.article.favorited" == false
jsonpath "$.article.favoritesCount" == 0
jsonpath "$.article.author.username" == "fav_{{uid}}"

POST {{host}}/api/users/{{user_id}}/mentored-by/{{mentor_id}}
HTTP 200

DELETE {{host}}/api/articles
HTTP 204

DELETE {{host}}/api/users
HTTP 204
