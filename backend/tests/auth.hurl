# =========================
# Health Check
# =========================
GET http://localhost:3000/checkhealth

HTTP/1.1 200


# =========================
# Register User
# =========================
POST http://localhost:3000/auth/register
Content-Type: application/json

{
  "email": "testuser@example.com",
  "name": "Test user"
}

HTTP/1.1 200
[Captures]
user_id: jsonpath "$.user_id"


# =========================
# Authorize (Login)
# =========================
POST http://localhost:3000/auth/authorize
Content-Type: application/json

{
  "email": "testuser@example.com"
}

HTTP/1.1 200
[Asserts]
jsonpath "$.access_token" exists

[Captures]
access_token: jsonpath "$.access_token"


# =========================
# Access Protected Route
# =========================
POST http://localhost:3000/auth/protected
Authorization: Bearer {{access_token}}

HTTP/1.1 200
[Asserts]
jsonpath "$.user_id" == "{{user_id}}"


# =========================
# Prune / Revoke User
# =========================
POST http://localhost:3000/auth/prune
Authorization: Bearer {{access_token}}

HTTP/1.1 200


# =========================
# Protected Route Should Fail After Prune
# =========================
POST http://localhost:3000/auth/protected
Authorization: Bearer {{access_token}}

HTTP/1.1 401
