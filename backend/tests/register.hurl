# Register user
POST http://localhost:3000/auth/register
Content-Type: application/json

{
  "email": "testuser@example.com",
  "name": "Test user"
}

HTTP/1.1 200
[Captures]
user_id: jsonpath "$.user_id"
