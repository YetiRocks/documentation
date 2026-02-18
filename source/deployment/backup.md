# Backup & Recovery

Yeti stores persistent data in RocksDB databases. In embedded mode, data lives on the local filesystem. In cluster mode, data is distributed across cluster nodes. This guide covers backup strategies, recovery procedures, and what can safely be regenerated.

## Data Locations

### Embedded Mode

```
{rootDirectory}/
├── data/                        # RocksDB databases (CRITICAL)
│   ├── yeti-auth/               # One directory per database name
│   │   ├── shard-0/
│   │   ├── shard-1/
│   │   ├── shard-2/
│   │   └── shard-3/
│   ├── yeti-telemetry/
│   └── ...
├── applications/                # Application configs and source (CRITICAL)
│   ├── yeti-auth/
│   │   ├── config.yaml
│   │   ├── schema.graphql
│   │   ├── resources/
│   │   └── data/
│   └── ...
├── cache/                       # Plugin build cache (REGENERATABLE)
│   └── builds/
│       └── {app-id}/
│           ├── src/
│           └── target/
├── certs/                       # TLS certificates
│   └── localhost/
│       ├── localhost-key.pem
│       └── localhost-cert.pem
└── yeti-config.yaml             # Server configuration (IMPORTANT)
```

### Cluster Mode

```
{rootDirectory}/
├── data/
│   └── cluster/                 # Cluster node data (managed by Docker volumes)
│       ├── docker-compose.yml
│       ├── pd1-data/
│       ├── pd2-data/
│       ├── pd3-data/
│       ├── node1-data/
│       ├── node2-data/
│       └── node3-data/
├── applications/                # Application configs and source (CRITICAL)
│   ├── yeti-auth/
│   │   ├── config.yaml
│   │   ├── schema.graphql
│   │   ├── resources/
│   │   └── data/
│   └── ...
├── cache/                       # Plugin build cache (REGENERATABLE)
│   └── builds/
│       └── {app-id}/
│           ├── src/
│           └── target/
├── certs/                       # TLS certificates
│   └── localhost/
│       ├── localhost-key.pem
│       └── localhost-cert.pem
└── yeti-config.yaml             # Server configuration (IMPORTANT)
```

In cluster mode, application data lives in the distributed cluster, not in local RocksDB files. The `data/cluster/` directory holds Docker volume mounts for the cluster nodes when using `autoStart: true`.

---

## What to Back Up

### Embedded Mode

| Directory | Priority | Regeneratable? |
|-----------|----------|---------------|
| `data/` | **Critical** | No -- contains all application data |
| `applications/` | **Critical** | No -- contains app configs and source |
| `yeti-config.yaml` | **Important** | No -- server configuration |
| `certs/` | **Important** | Yes if `autoGenerate: true`, but disrupts service |
| `cache/builds/` | Low | Yes -- recompiled on startup |

### Cluster Mode

| Item | Priority | Regeneratable? |
|------|----------|---------------|
| Cluster node data | **Critical** | No -- contains all application data |
| `applications/` | **Critical** | No -- contains app configs and source |
| `yeti-config.yaml` | **Important** | No -- server configuration |
| `certs/` | **Important** | Yes if `autoGenerate: true`, but disrupts service |
| `data/cluster/` | Low | Yes -- Docker volumes recreated by `autoStart` |
| `cache/builds/` | Low | Yes -- recompiled on startup |

---

## Embedded Mode Backup

### Hot Backup (Recommended)

RocksDB supports consistent reads while writes are occurring. You can safely copy the data directory while the server is running:

```bash
# Create a timestamped backup
BACKUP_DIR="/backups/yeti-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"

# Copy data directory (hot backup safe)
cp -r /var/lib/yeti/data "$BACKUP_DIR/data"

# Copy application configs
cp -r /var/lib/yeti/applications "$BACKUP_DIR/applications"

# Copy server config
cp /var/lib/yeti/yeti-config.yaml "$BACKUP_DIR/"
```

### Using rsync

For incremental backups:

```bash
rsync -av --delete \
  /var/lib/yeti/data/ \
  /backups/yeti-latest/data/

rsync -av --delete \
  /var/lib/yeti/applications/ \
  /backups/yeti-latest/applications/
```

### Automated Backups

Yeti has built-in backup configuration:

```yaml
maintenance:
  backup:
    enabled: true
    intervalHours: 24
    retentionDays: 30
```

For custom scheduling, use cron:

```bash
# /etc/cron.d/yeti-backup
0 2 * * * root /opt/yeti/scripts/backup.sh
```

---

## Cluster Mode Backup

When using cluster mode, application data is distributed across cluster nodes. Back up the cluster data volumes alongside the Yeti server configuration.

### Cluster Node Volumes

Back up the cluster data directories (or their Docker volumes):

```bash
BACKUP_DIR="/backups/yeti-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"

# Back up cluster data volumes
cp -r /var/lib/yeti/data/cluster "$BACKUP_DIR/cluster"

# Back up application configs
cp -r /var/lib/yeti/applications "$BACKUP_DIR/applications"

# Back up server config
cp /var/lib/yeti/yeti-config.yaml "$BACKUP_DIR/"
```

### Yeti Server Files

In cluster mode, the Yeti server itself is stateless (data lives in the cluster). Back up only the configuration:

```bash
BACKUP_DIR="/backups/yeti-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"

cp -r /var/lib/yeti/applications "$BACKUP_DIR/applications"
cp /var/lib/yeti/yeti-config.yaml "$BACKUP_DIR/"
```

---

## Recovery

### Full Recovery (Embedded Mode)

Restore from a backup to get a server running:

```bash
# Stop the server
kill $(pgrep yeti-core)

# Restore data
cp -r /backups/yeti-20240115-020000/data /var/lib/yeti/data

# Restore applications
cp -r /backups/yeti-20240115-020000/applications /var/lib/yeti/applications

# Restore config
cp /backups/yeti-20240115-020000/yeti-config.yaml /var/lib/yeti/

# Start the server (plugins will recompile from source)
yeti-core --root-dir /var/lib/yeti
```

The plugin cache (`cache/builds/`) does not need to be restored. It is regenerated automatically on startup (initial recompilation takes approximately 2 minutes per plugin).

### Partial Recovery (Single Database, Embedded Mode)

To restore just one application's data:

```bash
# Stop the server
kill $(pgrep yeti-core)

# Restore only the specific database
rm -rf /var/lib/yeti/data/my-app
cp -r /backups/yeti-latest/data/my-app /var/lib/yeti/data/my-app

# Restart
yeti-core --root-dir /var/lib/yeti
```

### Full Recovery (Cluster Mode)

1. Restore cluster data volumes from backup.
2. Restore Yeti server files:

```bash
cp -r /backups/yeti-latest/applications /var/lib/yeti/applications
cp /backups/yeti-latest/yeti-config.yaml /var/lib/yeti/

yeti-core --root-dir /var/lib/yeti
```

### Rebuilding Plugin Cache

If plugins fail to load after recovery, clear and rebuild:

```bash
rm -rf /var/lib/yeti/cache/builds/*/target/
rm -rf /var/lib/yeti/cache/builds/*/src/

# Restart - all plugins will recompile
yeti-core --root-dir /var/lib/yeti
```

This is especially important after a yeti-core binary upgrade, as the plugin ABI may have changed.

---

## Disaster Recovery Considerations

### Embedded Mode

- **RTO (Recovery Time Objective):** ~5 minutes (copy data + plugin recompile).
- **RPO (Recovery Point Objective):** Depends on backup frequency.
- **Storage format:** RocksDB SSTables are portable across machines with the same architecture.
- **No cross-architecture restore:** Backups from x86 cannot be restored on ARM (or vice versa) due to MessagePack encoding and RocksDB format differences.

### Cluster Mode

- **RTO:** Depends on cluster size and volume restore time.
- **RPO:** Cluster replicates data across nodes -- a single node failure causes no data loss.
- **High availability:** With 3+ replicas, the cluster tolerates minority node failures automatically.
- **Yeti server recovery:** Only `applications/` and `yeti-config.yaml` need restoring. Data lives in the cluster.

---

## Verifying Backups

After creating a backup, verify it by starting a test instance:

```bash
yeti-core --root-dir /backups/yeti-latest --apps yeti-auth
# Check that data is accessible
curl -sk https://localhost:9996/yeti-auth/auth
```
