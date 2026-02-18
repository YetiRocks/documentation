# OAuth Integration

Yeti supports OAuth 2.0 authentication with GitHub, Google, and Microsoft providers. Users authenticate through the third-party provider and receive a session cookie for subsequent requests.

## Setup

### 1. Register an OAuth Application

Register your application with the desired provider:

- **GitHub**: Settings > Developer settings > OAuth Apps
- **Google**: Google Cloud Console > APIs & Services > Credentials
- **Microsoft**: Azure Portal > App registrations

Set the callback URL to: `https://your-host:9996/yeti-auth/oauth_callback`

### 2. Set Environment Variables

Add your client credentials to the `.env` file in the Yeti root directory:

```bash
# GitHub OAuth
GITHUB_CLIENT_ID=your_github_client_id
GITHUB_CLIENT_SECRET=your_github_client_secret

# Google OAuth
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret

# Microsoft OAuth
MICROSOFT_CLIENT_ID=your_microsoft_client_id
MICROSOFT_CLIENT_SECRET=your_microsoft_client_secret
```

### 3. Configure Per-App OAuth Rules

Each application defines how OAuth users map to roles. Add rules in your app's `extensions:` config:

```yaml
# my-app/config.yaml
extensions:
  - yeti-auth:
      oauth:
        rules:
          - strategy: provider
            pattern: "google"
            role: admin
          - strategy: email
            pattern: "*@mycompany.com"
            role: standard
          - strategy: provider
            pattern: "github"
            role: standard
```

Rule strategies:
- **`provider`**: Match by OAuth provider name (`github`, `google`, `microsoft`).
- **`email`**: Match by email pattern with wildcard support.

Rules are evaluated in order. The first match determines the role. If no rule matches and no `default_role` is set, the user receives a `401 Unauthorized`.

## OAuth Flow

### 1. Initiate Login

Redirect the user to the OAuth login endpoint with the desired provider:

```
GET https://localhost:9996/yeti-auth/oauth_login?provider=github
```

This redirects the user to the provider's authorization page.

### 2. Callback

After the user authorizes, the provider redirects back to:

```
GET https://localhost:9996/yeti-auth/oauth_callback?code=AUTHORIZATION_CODE&state=CSRF_TOKEN
```

Yeti exchanges the code for tokens, creates a session, and sets a session cookie.

### 3. Use the Session

Subsequent requests include the session cookie automatically (browser) or manually:

```bash
curl -sk --cookie "session=SESSION_ID" \
  https://localhost:9996/my-app/MyTable
```

## Session Management

### Get Current User Info

```bash
curl -sk --cookie "session=SESSION_ID" \
  https://localhost:9996/yeti-auth/oauth_user
```

### Logout

```bash
curl -sk -X POST --cookie "session=SESSION_ID" \
  https://localhost:9996/yeti-auth/oauth_logout
```

### Refresh Provider Token

Refresh the OAuth provider's access token (for accessing provider APIs):

```bash
curl -sk -X POST --cookie "session=SESSION_ID" \
  https://localhost:9996/yeti-auth/oauth_refresh
```

## Session Storage

OAuth sessions are stored in a two-tier cache:
1. **In-memory cache**: Fast lookup for active sessions.
2. **Database fallback**: Sessions survive server restarts.

Session data includes the provider name, user email, provider access token, and the mapped Yeti role.

## CSRF Protection

The OAuth flow includes CSRF token protection:
- A unique token is generated for each login attempt.
- The token is included in the `state` parameter to the OAuth provider.
- On callback, the token is verified before completing authentication.
- Tokens expire after 10 minutes, with periodic cleanup every 100 insertions.

## Security

- OAuth callback URLs are validated at startup (SSRF protection: private IPs rejected, HTTPS required in production).
- Session cookies are httpOnly and secure.
- Provider tokens are stored server-side, never exposed to the client.

## See Also

- [Authentication Overview](auth-overview.md)
- [Roles & Permissions](auth-rbac.md) -- How roles map to access
- [Web Auth Demo](../examples/web-auth-demo.md) -- Working example application
