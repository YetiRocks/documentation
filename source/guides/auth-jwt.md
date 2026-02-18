# JWT Authentication

JSON Web Token (JWT) authentication provides stateless, token-based access for SPAs, mobile apps, and APIs. The client authenticates once and receives a token pair -- an access token for API requests and a refresh token for obtaining new access tokens.

## Login Flow

1. The client sends credentials to the login endpoint.
2. Yeti verifies the credentials and returns an `access_token` and `refresh_token`.
3. The client includes the access token in subsequent requests.
4. When the access token expires, the client uses the refresh token to get a new pair.

## Logging In

```bash
curl -sk -X POST https://localhost:9996/yeti-auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin"
  }'
```

Response:

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

## Making Authenticated Requests

Include the access token in the `Authorization` header:

```bash
curl -sk -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  https://localhost:9996/my-app/MyTable
```

### Shell Script Example

```bash
# Login and extract the access token
TOKEN=$(curl -sk -X POST https://localhost:9996/yeti-auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}' | jq -r .access_token)

# Use the token for API calls
curl -sk -H "Authorization: Bearer $TOKEN" \
  https://localhost:9996/my-app/MyTable

# Create a record
curl -sk -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"id": "item-1", "name": "Widget"}' \
  https://localhost:9996/my-app/MyTable
```

## Embedded Permissions

The access token contains embedded permissions from the user's role. This means the `JwtAuthProvider` does not need to query the database on every request -- the permissions are decoded directly from the token. This makes JWT authentication the fastest auth method in Yeti.

## Refreshing Tokens

When the access token expires, use the refresh token to obtain a new token pair without re-entering credentials:

```bash
curl -sk -X POST https://localhost:9996/yeti-auth/jwt_refresh \
  -H "Content-Type: application/json" \
  -d '{
    "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }'
```

Response:

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

## Checking Auth Status

Verify that a token is valid and see the current user's identity:

```bash
curl -sk -H "Authorization: Bearer $TOKEN" \
  https://localhost:9996/yeti-auth/auth
```

## JavaScript Client Example

```javascript
async function login(username, password) {
  const res = await fetch('https://localhost:9996/yeti-auth/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, password }),
  });
  return res.json(); // { access_token, refresh_token, expires_in }
}

async function fetchData(token) {
  const res = await fetch('https://localhost:9996/my-app/MyTable', {
    headers: { 'Authorization': `Bearer ${token}` },
  });
  return res.json();
}

// Usage
const auth = await login('admin', 'admin');
const data = await fetchData(auth.access_token);
```

## Security Considerations

- Access tokens are short-lived (default 1 hour). Keep them in memory, not localStorage.
- Refresh tokens are longer-lived. Store them securely (httpOnly cookie or secure storage).
- If a token is compromised, changing the user's password invalidates all tokens.
- JWT validation happens entirely in-memory with no database call, making it suitable for high-throughput APIs.

## See Also

- [Authentication Overview](auth-overview.md)
- [Basic Authentication](auth-basic.md) -- Simpler credential-per-request auth
- [OAuth Integration](auth-oauth.md) -- Third-party provider auth
