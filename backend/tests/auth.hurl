GET http://localhost:{{PORT}}/checkhealth
HTTP 200


# POST http://localhost:{{PORT}}/auth/register
# Content-Type: application/json
# 
# {
#   "email": "john@example.com",
#   "name": "John Doe"
# }
# 
# HTTP 200
# [Asserts]
# jsonpath "$.user_id" exists


POST http://localhost:{{PORT}}/auth/authorize
Content-Type: application/json

{
  "email": "john@example.com"
}

HTTP 200
