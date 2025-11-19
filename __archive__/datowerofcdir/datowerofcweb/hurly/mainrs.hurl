GET http://localhost:9112
HTTP/1.1 200
[Asserts]
xpath "normalize-space(//head/title)" == "datowerofcweb 0.1.0"
xpath "normalize-space(//body/h1)" == "datowerofcweb 0.1.0"