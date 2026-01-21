# Verify code and obtain token
# Requires variable `code` to be passed when running hurl (see README snippet below)
POST http://localhost:3000/auth/verify_code
Content-Type: application/json

{
  "email": "{{email}}",
  "code": "{{code}}"
}

HTTP/1.1 200

[Captures]
access_token: jsonpath "$.access_token"