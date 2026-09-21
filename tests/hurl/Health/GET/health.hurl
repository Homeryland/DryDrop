GET http://localhost:7878/api/v1/health
HTTP 200
[Asserts]
status == 200
jsonpath "$.code" == 200
jsonpath "$.message" == "healthy"
