# Development Setup

**Set up your local development environment for contributing to Yeti**

This guide covers everything you need to build, test, and develop Yeti on your local machine.

## Prerequisites

### Required

- **Rust**: 1.75 or higher
- **Git**: For version control
- **Operating System**: Linux, macOS, or Windows (WSL2)

### Optional but Recommended

- **Docker**: For cluster mode (auto-start) and Qdrant (RAG service)
- **VSCode** or **IntelliJ IDEA**: With Rust support

---

## Initial Setup

### 1. Install Rust

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version  # Should be 1.75+
cargo --version
```

### 2. Clone Repository

```bash
# Clone Yeti
git clone https://github.com/your-org/yeti.git
cd yeti

# Check current branch
git branch
```

### 3. Build Project

```bash
# Build all workspace crates
cargo build --release

# This will take a few minutes on first build
# Subsequent builds are much faster
```

### 4. Run Tests

```bash
# Run all tests
cargo test --workspace

# Expected: 191+ tests passing
```

---

## IDE Setup

### Visual Studio Code

**Install Extensions**:
```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",    // Rust language support
    "vadimcn.vscode-lldb",         // Debugging
    "tamasfe.even-better-toml",    // TOML support
    "serayuzgur.crates"            // Cargo.toml management
  ]
}
```

**Configure Rust Analyzer** (`.vscode/settings.json`):
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "rust-analyzer.procMacro.enable": true
}
```

**Keyboard Shortcuts**:
- `Ctrl+Shift+B` - Build
- `F5` - Debug
- `Ctrl+Shift+T` - Run tests

### IntelliJ IDEA / RustRover

**Install Plugin**:
1. Settings → Plugins
2. Search "Rust"
3. Install and restart

**Configure**:
- Settings → Languages & Frameworks → Rust
- Enable "Use clippy" for linting
- Enable "External linter" for cargo check

---

## Development Commands

### Building

```bash
# Fast compile check (no binary)
cargo check

# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Build specific crate
cargo build -p yeti-core
```

### Testing

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p yeti-core

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored
```

### Linting

```bash
# Run Clippy (linter)
cargo clippy --workspace

# Fix automatically fixable issues
cargo clippy --workspace --fix

# Pedantic lints (strict)
cargo clippy --workspace -- -W clippy::pedantic
```

### Formatting

```bash
# Check formatting
cargo fmt --all -- --check

# Format code
cargo fmt --all
```

### Documentation

```bash
# Build documentation
cargo doc --no-deps

# Build and open in browser
cargo doc --no-deps --open

# Check for broken links
cargo doc --no-deps 2>&1 | grep warning
```

### Benchmarks

```bash
# Run all benchmarks
cargo bench

# Specific benchmark
cargo bench --bench integration

# Save baseline
cargo bench -- --save-baseline before-changes

# Compare to baseline
cargo bench -- --baseline before-changes
```

---

## RAG Service Setup (Optional)

The RAG service allows querying Harper's source code during development.

### Start Qdrant

```bash
# Using Docker
docker run -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_storage:/qdrant/storage \
    qdrant/qdrant

# Verify it's running
curl http://localhost:6334/health
```

### Build MCP Server

```bash
# Build the MCP stdio server
cargo build --release --bin mcp-stdio-server

# Binary location
./target/release/mcp-stdio-server
```

**See**: [RAG Service Guide](../developers/rag-service.md) for complete usage instructions.

---

## Running Applications

### Application Template

```bash
# Navigate to template
cd applications/application-template

# Run in development mode
cargo run -- \
  --config config.yaml \
  --db-path /tmp/yeti-db \
  --http-bind 127.0.0.1:9996

# Test it works
curl http://localhost:9996/TableName
```

### Your Own Application

```bash
# Create new application from template
cp -r applications/application-template applications/my-app
cd applications/my-app

# Modify config.yaml and schema.graphql
# Then run
cargo run --release
```

---

## Troubleshooting

### Build Errors

**Error: `linker 'cc' not found`**
```bash
# macOS
xcode-select --install

# Linux (Ubuntu/Debian)
sudo apt install build-essential

# Linux (Fedora)
sudo dnf install gcc
```

**Error: `failed to load source for dependency`**
```bash
# Clean and rebuild
cargo clean
cargo build
```

### Test Failures

**Timeout errors**:
```bash
# Increase timeout
RUST_TEST_THREADS=1 cargo test
```

**Database lock errors**:
```bash
# Clean test databases
rm -rf /tmp/yeti-test-*
cargo test
```

### RAG Service Issues

**Qdrant not responding**:
```bash
# Check if running
docker ps | grep qdrant

# Restart
docker restart qdrant

# Check logs
docker logs qdrant
```

### Performance Issues

**Slow builds**:
```bash
# Use faster linker (macOS/Linux)
# Add to ~/.cargo/config.toml:
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache
```

---

## Development Workflow

### Typical Session

```bash
# 1. Pull latest changes
git pull origin main

# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Make changes and test frequently
cargo check         # Fast feedback
cargo test          # Run tests
cargo clippy        # Check lints

# 4. Before committing
cargo test --workspace  # All tests
cargo clippy --workspace # All lints
cargo fmt --all     # Format code

# 5. Commit (human reviews before pushing)
git add .
git commit -m "feat: implement my feature"
```

### Quality Checks

Before submitting PR:

```bash
# All tests pass
cargo test --workspace

# No clippy warnings
cargo clippy --workspace

# Code is formatted
cargo fmt --all -- --check

# Documentation builds
cargo doc --no-deps

# Benchmarks don't regress (if applicable)
cargo bench
```

---

## Environment Variables

Useful environment variables for development:

```bash
# Rust backtrace on panic
export RUST_BACKTRACE=1          # Short backtrace
export RUST_BACKTRACE=full       # Full backtrace

# Logging level
export RUST_LOG=debug            # Debug logs
export RUST_LOG=yeti_core=trace  # Trace for specific crate

# Test output
export RUST_TEST_NOCAPTURE=1     # Show test output

# Build parallelism
export CARGO_BUILD_JOBS=4        # Limit build jobs
```

---

## Project Structure

```
yeti/
├── Cargo.toml                 # Workspace root
├── platform/
│   ├── yeti-core/            # Core library
│   ├── yeti-builder/         # Development tools
│   └── yeti-macros/          # Procedural macros
├── applications/
│   └── application-template/ # Reference implementation
├── documentation/            # This documentation
├── tasks/                    # Feature tracking
└── benches/                  # Benchmarks
```

---

## Getting Help

- **[Discord Community](https://discord.gg/VzZuaw3Xay)** - Real-time help
- **[GitHub Discussions](https://github.com/harperfast/yeti/discussions)** - Technical questions
- **[Contributing Guide](contributing.md)** - Contribution process
- **[Architecture](../foundations/architecture.md)** - System design

---

## Next Steps

Once your environment is set up:

1. **[Read the Workflow Guide](../developers/workflow.md)** - Learn the development process
2. **[Check Tasks](../../tasks/)** - Find something to work on
3. **[Join Discord](https://discord.gg/VzZuaw3Xay)** - Introduce yourself

**Ready to contribute? See [Contributing Guide](contributing.md) for the process.**
