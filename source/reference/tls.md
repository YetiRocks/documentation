# TLS & HTTPS

Yeti serves its application API over HTTPS by default on port 9996. TLS is implemented using Rustls with the ring cryptography provider -- no OpenSSL dependency is required.

---

## Configuration

TLS settings are in the `tls` section of `yeti-config.yaml`:

```yaml
tls:
  autoGenerate: false
  privateKey: null
  certificate: null
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `tls.autoGenerate` | boolean | `false` | Auto-generate self-signed certificates on startup |
| `tls.privateKey` | string | `null` | Path to PEM-encoded private key file |
| `tls.certificate` | string | `null` | Path to PEM-encoded certificate file (or chain) |

---

## Auto-Generated Self-Signed Certificates

For development, Yeti can generate self-signed certificates automatically:

```yaml
tls:
  autoGenerate: true
```

Certificates are created at startup and stored in `$rootDirectory/certs/localhost/`:
- `localhost-key.pem` -- Private key
- `localhost-cert.pem` -- Self-signed certificate

These certificates are valid for `localhost` and `127.0.0.1`.

---

## Manual Certificates

For production or custom domains, provide your own certificates:

```yaml
tls:
  privateKey: /etc/ssl/private/yeti.key
  certificate: /etc/ssl/certs/yeti.crt
```

Certificate requirements:
- PEM format
- The certificate file can contain a full chain (server cert + intermediates)
- RSA (2048+ bit) or ECDSA (P-256, P-384) keys are supported
- The private key must not be password-protected

### Using Let's Encrypt

```bash
# Generate certificates with certbot
sudo certbot certonly --standalone -d yeti.example.com

# Reference in config
# tls:
#   privateKey: /etc/letsencrypt/live/yeti.example.com/privkey.pem
#   certificate: /etc/letsencrypt/live/yeti.example.com/fullchain.pem
```

### Using mkcert (Local Development)

```bash
# Install mkcert
brew install mkcert  # macOS
mkcert -install

# Generate certificates
mkcert localhost 127.0.0.1

# Reference in config
# tls:
#   privateKey: ./localhost+1-key.pem
#   certificate: ./localhost+1.pem
```

---

## Development Workflow

Self-signed certificates cause TLS verification failures in most HTTP clients. Use the `-k` flag with curl to skip verification:

```bash
# Always use -sk with self-signed certs
curl -sk https://localhost:9996/my-app/TableName

# Or set it as a default alias
alias curls='curl -sk'
curls https://localhost:9996/my-app/TableName
```

For browser access, you will need to accept the self-signed certificate warning or install it as a trusted root certificate.

---

## Operations API (No TLS)

The Operations API on port 9995 runs over plain HTTP without TLS. This is intentional -- the operations API is designed for local or internal network access only.

```bash
# Operations API uses HTTP, not HTTPS
curl http://localhost:9995/health
curl -X POST http://localhost:9995/ \
  -H "Content-Type: application/json" \
  -d '{"operation": "health_check"}'
```

Do not expose port 9995 to the public internet. In production, restrict it to localhost or an internal network using firewall rules.

---

## TLS Implementation Details

Yeti uses the following TLS stack:

| Component | Library | Notes |
|-----------|---------|-------|
| TLS protocol | Rustls | Pure-Rust TLS implementation |
| Cryptography | ring | Audited cryptographic primitives |
| Certificate parsing | rustls-pemfile | PEM file handling |

This stack has no dependency on OpenSSL or any system TLS library. It compiles and runs identically across Linux, macOS, and Windows.

Supported protocol versions:
- TLS 1.2
- TLS 1.3 (preferred)

Supported cipher suites (TLS 1.3):
- `TLS_AES_256_GCM_SHA384`
- `TLS_AES_128_GCM_SHA256`
- `TLS_CHACHA20_POLY1305_SHA256`

---

## Troubleshooting

### "Connection refused" on port 9996

Ensure the server started successfully. Check logs for TLS initialization errors:

```bash
# Check if the server is running
curl -sk https://localhost:9996/

# Check operations API (no TLS required)
curl http://localhost:9995/health
```

### "Certificate not trusted"

This is expected with self-signed certificates. Use `curl -sk` or install the certificate as a trusted root.

### "Private key does not match certificate"

Verify the key and certificate belong together:

```bash
# Compare modulus hashes (should match for RSA keys)
openssl x509 -noout -modulus -in cert.pem | openssl md5
openssl rsa -noout -modulus -in key.pem | openssl md5
```

---

## See Also

- [Server Configuration](server-config.md) -- Complete config reference
- [Environment Variables](environment-variables.md) -- Environment setup
- [CLI Arguments](cli.md) -- Command-line options
