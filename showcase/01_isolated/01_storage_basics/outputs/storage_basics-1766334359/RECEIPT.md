# NestGate Storage Basics - Demo Receipt

**Date**: Sun Dec 21 11:26:05 AM EST 2025
**Duration**: 6s
**Backend**: Filesystem (no-sudo mode)
**Status**: ✅ SUCCESS

---

## Operations Performed

### 1. Storage Backend Creation
- Created filesystem storage: `outputs/storage_basics-1766334359/storage`
- Simulated 1GB pool capacity
- Storage path: `outputs/storage_basics-1766334359/storage`

### 2. Dataset Creation
- `data` (256MB quota simulated)
- `logs` (256MB quota simulated)
- `backups` (256MB quota simulated)
- `cache` (256MB quota simulated)
- `tmp` (256MB quota simulated)

### 3. Test Data Written
- Test file: `outputs/storage_basics-1766334359/storage/data/test-1766334359.bin` (10MB)
- File hash: `d602a4f58a98f77618cbc2553d974301...`
- Log files: 5 sample files created
- Backup manifest: Created
- Total data written: ~10MB

### 4. Snapshots Created
- `backups-snapshot-1766334359.tar.gz` (4.0K)
- `cache-snapshot-1766334359.tar.gz` (4.0K)
- `data-snapshot-1766334359.tar.gz` (11M)
- `logs-snapshot-1766334359.tar.gz` (4.0K)
- `tmp-snapshot-1766334359.tar.gz` (4.0K)

### 5. Performance Metrics
- Total storage used: 11M
- Total files: 12
- Datasets: 5
- Snapshots: 5

---

## Files Generated

```
-rw-rw-r-- 1 eastgate eastgate  386 Dec 21 11:26 datasets.txt
-rw-rw-r-- 1 eastgate eastgate   40 Dec 21 11:26 disk-usage.txt
-rw-rw-r-- 1 eastgate eastgate  214 Dec 21 11:25 pool-info.json
-rw-rw-r-- 1 eastgate eastgate    0 Dec 21 11:26 RECEIPT.md
drwxrwxr-x 2 eastgate eastgate 4.0K Dec 21 11:26 snapshots
-rw-rw-r-- 1 eastgate eastgate  413 Dec 21 11:26 snapshots.txt
-rw-rw-r-- 1 eastgate eastgate  171 Dec 21 11:26 stats.txt
drwxrwxr-x 7 eastgate eastgate 4.0K Dec 21 11:26 storage
-rw-rw-r-- 1 eastgate eastgate  260 Dec 21 11:26 storage-usage.txt
```

## Storage Structure

```
outputs/storage_basics-1766334359/storage
outputs/storage_basics-1766334359/storage/backups
outputs/storage_basics-1766334359/storage/backups/snapshots
outputs/storage_basics-1766334359/storage/data
outputs/storage_basics-1766334359/storage/logs
outputs/storage_basics-1766334359/storage/cache
outputs/storage_basics-1766334359/storage/tmp
```

---

## Cleanup

```bash
rm -rf outputs/storage_basics-1766334359
```

---

## Key Capabilities Demonstrated

- ✅ Storage backend creation
- ✅ Dataset organization (5 datasets)
- ✅ Real data write operations (10MB)
- ✅ Snapshot/backup functionality
- ✅ Performance monitoring
- ✅ Data integrity verification

---

## NestGate Features Shown

1. **Universal Storage Backend**: Filesystem-based storage
2. **Graceful Degradation**: Works without ZFS
3. **Data Organization**: Logical dataset structure
4. **Snapshot Support**: Tar-based snapshots
5. **Performance Monitoring**: Real-time statistics
6. **Integrity Checking**: Hash verification

---

**Demo Version**: 1.0.0
**Generated**: Sun Dec 21 11:26:05 AM EST 2025
