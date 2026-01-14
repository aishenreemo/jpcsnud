# Access protected route
# Requires `access_token` and `user_id` variables to be provided
POST http://localhost:3000/auth/protected
Authorization: Bearer {{access_token}}

HTTP/1.1 200
[Asserts]
jsonpath "$.user_id" == "{{user_id}}"
