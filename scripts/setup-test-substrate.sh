#!/usr/bin/env bash
set -euo pipefail

# NestGate multi-filesystem test substrate setup
# Requires: btrfs-progs, xfsprogs, zfsutils-linux
# All drives listed are explicitly dedicated to testing.

MOUNT_BASE="/mnt/nestgate"

echo "=== NestGate Test Substrate Setup ==="
echo "Creating mount hierarchy at ${MOUNT_BASE}"

mkdir -p "${MOUNT_BASE}/cold/zfs"
mkdir -p "${MOUNT_BASE}/cold/btrfs"
mkdir -p "${MOUNT_BASE}/cold/xfs"
mkdir -p "${MOUNT_BASE}/cold/ext4"
mkdir -p "${MOUNT_BASE}/warm"

# --- Phase 1: Wipe stale ZFS labels from drives being repurposed ---
echo ""
echo "--- Phase 1: Wiping stale partition tables on sdc, sdd, sde ---"
for disk in sdc sdd sde; do
    echo "  Wiping /dev/${disk}..."
    wipefs -a "/dev/${disk}" || true
    sgdisk --zap-all "/dev/${disk}" || true
done

# --- Phase 2: Create single partitions on repurposed drives ---
echo ""
echo "--- Phase 2: Creating partitions ---"

echo "  sdc -> btrfs partition"
sgdisk -n 1:0:0 -t 1:8300 "/dev/sdc"
partprobe /dev/sdc
sleep 1

echo "  sdd -> xfs partition"
sgdisk -n 1:0:0 -t 1:8300 "/dev/sdd"
partprobe /dev/sdd
sleep 1

echo "  sde -> ext4 partition"
sgdisk -n 1:0:0 -t 1:8300 "/dev/sde"
partprobe /dev/sde
sleep 1

# --- Phase 3: Format filesystems ---
echo ""
echo "--- Phase 3: Formatting ---"

echo "  Formatting sdc1 as btrfs..."
mkfs.btrfs -f -L nestgate-btrfs /dev/sdc1

echo "  Formatting sdd1 as xfs..."
mkfs.xfs -f -L nestgate-xfs /dev/sdd1

echo "  Formatting sde1 as ext4..."
mkfs.ext4 -F -L nestgate-ext4 /dev/sde1

# --- Phase 4: ZFS pool (sda + sdb mirror, sdf hot spare) ---
echo ""
echo "--- Phase 4: ZFS pool setup ---"

# Wipe stale labels on ZFS drives (they already have zfs_member but from old pool)
for disk in sda sdb sdf; do
    echo "  Wiping stale ZFS labels on /dev/${disk}..."
    zpool labelclear -f "/dev/${disk}1" 2>/dev/null || true
done

echo "  Creating ZFS mirror pool 'nestgate' with hot spare..."
zpool create -f \
    -o ashift=12 \
    -O compression=lz4 \
    -O atime=off \
    -O xattr=sa \
    -O dnodesize=auto \
    -O normalization=formD \
    -O mountpoint="${MOUNT_BASE}/cold/zfs" \
    nestgate mirror /dev/sda /dev/sdb spare /dev/sdf

echo "  Creating ZFS datasets..."
zfs create nestgate/data
zfs create nestgate/snapshots
zfs create nestgate/cache
zfs create nestgate/testing

echo "  ZFS pool status:"
zpool status nestgate

# --- Phase 5: Mount non-ZFS filesystems ---
echo ""
echo "--- Phase 5: Mounting ---"

mount /dev/sdc1 "${MOUNT_BASE}/cold/btrfs"
mount -t xfs /dev/sdd1 "${MOUNT_BASE}/cold/xfs"
mount /dev/sde1 "${MOUNT_BASE}/cold/ext4"

# Create btrfs subvolumes for nestgate
btrfs subvolume create "${MOUNT_BASE}/cold/btrfs/data"
btrfs subvolume create "${MOUNT_BASE}/cold/btrfs/snapshots"

# Create directory structure on xfs and ext4
for fs in xfs ext4; do
    mkdir -p "${MOUNT_BASE}/cold/${fs}/data"
    mkdir -p "${MOUNT_BASE}/cold/${fs}/snapshots"
done

# Symlink warm tier
ln -sf / "${MOUNT_BASE}/warm/ext4-nvme"

# --- Phase 6: Set ownership ---
echo ""
echo "--- Phase 6: Setting ownership ---"
chown -R westgate:westgate "${MOUNT_BASE}"

# --- Summary ---
echo ""
echo "=== Substrate Ready ==="
echo ""
echo "Warm tier (NVMe SSD):"
echo "  ext4:  / (1.8TB Samsung 970 EVO Plus)"
echo ""
echo "Cold tier (HDD):"
echo "  ZFS:   ${MOUNT_BASE}/cold/zfs (sda+sdb mirror, sdf spare)"
echo "  btrfs: ${MOUNT_BASE}/cold/btrfs (sdc, 12.7TB)"
echo "  xfs:   ${MOUNT_BASE}/cold/xfs (sdd, 12.7TB)"
echo "  ext4:  ${MOUNT_BASE}/cold/ext4 (sde, 12.7TB)"
echo ""
df -h "${MOUNT_BASE}/cold/btrfs" "${MOUNT_BASE}/cold/xfs" "${MOUNT_BASE}/cold/ext4" 2>/dev/null
zpool list nestgate 2>/dev/null
echo ""
echo "Done. NestGate can now test across all substrate types."
