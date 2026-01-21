# Prune / Revoke user
# Requires `access_token` variable
POST http://localhost:3000/auth/prune
Authorization: Bearer {{access_token}}

HTTP/1.1 200
