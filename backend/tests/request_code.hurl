# Request verification code (dev: EXPOSE_LOGIN_CODE=1 to return code in response)
POST http://localhost:3000/auth/request_code
Content-Type: application/json

{
  "email": "{{email}}"
}

HTTP/1.1 200