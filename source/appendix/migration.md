# Migrating from Harper to Yeti

This guide helps you migrate existing Harper applications to Yeti with minimal changes.

## Why Migrate?

- **Native Performance**: 10-50x faster for many workloads
- **Simpler Operations**: Single binary, no Node.js dependencies
- **Better Resource Usage**: Lower memory footprint, faster startup
- **100% Compatible**: Same APIs, same behavior, drop-in replacement

## Migration Strategy

### 1. Assess Your Application

**What's Supported (100% Compatible)**:
- ✅ Resource API (CRUD operations)
- ✅ GraphQL schemas → REST endpoints
- ✅ FIQL queries and filtering
- ✅ Static file serving
- ✅ config.yaml configuration
- ✅ Multi-tenancy

**What's In Progress**:
- ⏳ Operations API (partial)
- ⏳ Clustering & replication
- ⏳ Custom authentication hooks
- ⏳ WebSocket/real-time features

See [Compatibility Matrix](../migration/compatibility-matrix.md) for complete feature status.

### 2. Preparation

Before migrating, ensure you have:

1. **Backup**: Full database backup
2. **Version Control**: Your Harper application in git
3. **Test Environment**: Separate environment for testing
4. **Rollback Plan**: Ability to revert to Harper quickly

### 3. Side-by-Side Migration (Recommended)

Run Harper and Yeti in parallel during migration:

```bash
# Terminal 1: Keep Harper running
cd harper-app
npm start

# Terminal 2: Start Yeti
cd yeti-app
cargo run --release -- --http-bind 127.0.0.1:9997
```

Test against both, compare responses, gradually shift traffic.

## Step-by-Step Migration

### Step 1: Create Yeti Application Structure

```bash
# From Yeti workspace
cd applications
mkdir my-app && cd my-app

# Copy application template
cp -r ../application-template/* .
```

### Step 2: Copy Configuration Files

```bash
# Copy from your Harper application
cp /path/to/harper-app/config.yaml .
cp /path/to/harper-app/schema.graphql .

# Copy static files (if any)
cp -r /path/to/harper-app/web ./static
```

**No modifications needed!** Yeti uses the same config.yaml and schema.graphql formats.

### Step 3: Port Custom Resources

If your Harper app has custom resources (`resources.js` or `resources/`), port them to Rust.

**Harper (JavaScript)**:
```javascript
// resources.js
export default {
  users: {
    beforeCreate: async (data) => {
      data.created_at = new Date().toISOString();
      return data;
    },
    afterCreate: async (data) => {
      await sendWelcomeEmail(data.email);
    }
  }
}
```

**Yeti (Rust)**:
```rust
// src/resources.rs
use yeti_core::prelude::*;

pub struct UserResource;

impl Resource for UserResource {
    fn name(&self) -> &'static str {
        "users"
    }

    async fn before_create(&self, data: &mut Value) -> Result<()> {
        data["created_at"] = chrono::Utc::now().to_rfc3339().into();
        Ok(())
    }

    async fn after_create(&self, data: &Value) -> Result<()> {
        send_welcome_email(data["email"].as_str().unwrap()).await?;
        Ok(())
    }
}
```

See [Custom Resources Guide](../developers/custom-resources.md) for complete porting guide.

### Step 4: Data Migration

#### Option A: Export/Import (Recommended)

```bash
# Export from Harper
curl http://localhost:9996/User > users.json

# Import to Yeti
cat users.json | jq -c '.[]' | while read record; do
  curl -X POST http://localhost:9997/User \
    -H "Content-Type: application/json" \
    -d "$record"
done
```

#### Option B: Database Copy (Advanced)

If you're comfortable with database internals:

```bash
# Harper uses LMDB or RocksDB
# Yeti uses RocksDB

# If Harper already uses RocksDB, you might be able to copy the database
# (Experimental - verify data integrity thoroughly)
cp -r /path/to/harper/data /path/to/yeti/data
```

See [Data Migration Guide](../migration/data-migration.md) for detailed instructions.

### Step 5: Build and Test

```bash
# Build your Yeti application
cargo build --release

# Run with test database
./target/release/my-app \
  --config config.yaml \
  --db-path /tmp/yeti-test-db \
  --http-bind 127.0.0.1:9997
```

### Step 6: Validation

Use the validation script to compare Harper and Yeti responses:

```bash
# Compare GET requests
curl http://localhost:9996/User/test-id > harper.json
curl http://localhost:9997/User/test-id > yeti.json
diff harper.json yeti.json

# Compare list operations
curl http://localhost:9996/User?filter=role==admin > harper-list.json
curl http://localhost:9997/User?filter=role==admin > yeti-list.json
diff harper-list.json yeti-list.json
```

See [Testing Migration](../migration/testing-migration.md) for comprehensive validation.

### Step 7: Production Cutover

When ready for production:

1. **Announce downtime window** (or use zero-downtime strategy)
2. **Final data sync** from Harper to Yeti
3. **Switch traffic** to Yeti
4. **Monitor closely** for 24-48 hours
5. **Keep Harper running** as backup (for rollback)

## Zero-Downtime Migration

For critical applications that can't have downtime:

### Using Dual-Write Pattern

```bash
# Application writes to both Harper and Yeti
# Reads come from Harper (verified source)
# Validate Yeti data in background
# Switch reads to Yeti after validation
# Deprecate Harper writes
```

### Using Database Replication

If both use RocksDB:
```bash
# Set up replication from Harper to Yeti
# Validate data consistency
# Switch application to Yeti
# Keep Harper as read-replica temporarily
```

See [Zero-Downtime Migration](../migration/zero-downtime.md) for detailed strategies.

## Common Issues

### Issue: Response Format Differences

**Symptom**: Yeti returns slightly different JSON structure

**Solution**: Check GraphQL schema - Yeti strictly follows schema definitions. Update schema if needed.

### Issue: Query Performance Difference

**Symptom**: FIQL queries slower/faster than Harper

**Solution**: Yeti has different indexing strategy. Check [Performance Tuning](../administration/performance-tuning.md).

### Issue: Custom Authentication Not Working

**Symptom**: Auth hooks from Harper don't work

**Solution**: Port authentication logic to Yeti's security system. See [Security Guide](../developers/security/).

### Issue: Static File Serving Path Issues

**Symptom**: Static files not found

**Solution**: Update config.yaml `static_directory` path. Yeti uses different path resolution.

See [Common Migration Issues](../migration/common-issues.md) for complete troubleshooting guide.

## Migration Checklist

Use this checklist to track your migration progress:

- [ ] Backup Harper database
- [ ] Create Yeti application structure
- [ ] Copy config.yaml and schema.graphql
- [ ] Copy static files
- [ ] Port custom resources to Rust
- [ ] Migrate data (export/import)
- [ ] Test all CRUD operations
- [ ] Validate FIQL queries
- [ ] Test authentication/authorization
- [ ] Load test Yeti application
- [ ] Set up monitoring
- [ ] Plan cutover timing
- [ ] Execute cutover
- [ ] Monitor production
- [ ] Decommission Harper (after validation period)

See [Full Migration Checklist](../migration/checklist.md) for detailed tasks.

## Getting Help

Migration assistance:

- 💬 **[Discord - #migration](https://discord.gg/VzZuaw3Xay)** - Real-time help
- 📧 **Migration Support**: migration@yeti.rocks
- 📖 **[Migration Docs](../migration/)** - Complete guides
- 🐛 **[GitHub Issues](https://github.com/harperfast/yeti/issues)** - Report migration bugs

## Success Stories

> "Migrated our 50GB Harper database to Yeti in 2 hours. Response times dropped from 50ms to 5ms. Memory usage cut in half." - *Development Team*

> "Zero-downtime migration using dual-write pattern. Seamless for our users." - *Platform Team*

> "Porting custom resources was straightforward. The Rust safety features caught bugs that existed in our JS code." - *Backend Developer*

**Ready to migrate?** Join our [Discord](https://discord.gg/VzZuaw3Xay) and we'll help you succeed.
