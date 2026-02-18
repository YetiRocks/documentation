# Installation

Get Yeti running on your local machine.

## Prerequisites

- **Rust** 1.91 or higher ([install via rustup](https://rustup.rs/))
- **Git** for cloning the repository
- **Operating System**: macOS (ARM or x86), Linux (ARM or x86), or Windows (WSL2)
- **Docker** (optional, for cluster mode)

Pre-built RocksDB binaries are included for supported platforms. No C++ toolchain required.

## Clone and Build

```bash
git clone https://github.com/yetiRocks/yeti.git ~/Developer/yeti
cd ~/Developer/yeti
cargo build --release
```

The binary is at `target/release/yeti-core`.

## Set Up the Runtime Directory

Yeti separates source code from runtime data. The runtime directory holds configuration, applications, certificates, and databases.

```bash
# Create runtime directory (default: ~/yeti)
mkdir -p ~/yeti/applications ~/yeti/certs ~/yeti/data
```

Copy the default configuration:

```bash
cp ~/Developer/yeti/yeti-config.yaml ~/yeti/yeti-config.yaml
```

## Configure

Edit `~/yeti/yeti-config.yaml`:

```yaml
environment: development
rootDirectory: ~/yeti

http:
  port: 9996

operationsApi:
  port: 9995
  enabled: true

storage:
  caching: true

logging:
  level: info

tls:
  autoGenerate: true
```

With `tls.autoGenerate: true`, Yeti creates self-signed certificates on first startup.

## Configure Secrets

Create a `.env` file at the source root for API keys:

```bash
cp ~/Developer/yeti/.env.example ~/Developer/yeti/.env
```

Edit `.env` to add your keys (all optional):

```bash
JWT_SECRET_KEY=generate-a-secure-random-key
GOOGLE_CLIENT_ID=your-client-id
GOOGLE_CLIENT_SECRET=your-client-secret
GITHUB_CLIENT_ID=your-client-id
GITHUB_CLIENT_SECRET=your-client-secret
```

## Start the Server

```bash
cd ~/Developer/yeti
cargo run --release
```

Or use the binary directly:

```bash
~/Developer/yeti/target/release/yeti-core
```

Override the root directory or filter applications:

```bash
yeti-core --root-dir ~/yeti --apps application-template,realtime-demo
```

## Verify

```bash
# Test with self-signed cert (-sk skips cert verification)
curl -sk https://localhost:9996/application-template/TableName

# Should return: []
```

## Create Your First Record

```bash
curl -sk -X POST https://localhost:9996/application-template/TableName \
  -H "Content-Type: application/json" \
  -d '{"id": "test-1", "count": 42}'

curl -sk https://localhost:9996/application-template/TableName/test-1
# Returns: {"id": "test-1", "count": 42}
```

## Platform Notes

### macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Linux

```bash
sudo apt update && sudo apt install -y build-essential pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows (WSL2)

```powershell
wsl --install -d Ubuntu
# Then follow Linux instructions inside WSL2
```

## Troubleshooting

**`linker 'cc' not found`** -- Install build tools: `xcode-select --install` (macOS) or `sudo apt install build-essential` (Linux).

**`Address already in use`** -- Another yeti-core process is running. Yeti automatically kills existing instances on startup, but you can check with `lsof -i :9996`.

**Plugin compilation slow** -- First build compiles plugins (~2 min each, ~6 min total). Subsequent restarts use cached dylibs (~10 seconds).

## Next Steps

- [Quickstart](quickstart.md) -- Build a REST API in 5 minutes
- [Your First Application](first-application.md) -- Detailed tutorial
- [Core Concepts](../concepts/applications.md) -- Understand how Yeti works
