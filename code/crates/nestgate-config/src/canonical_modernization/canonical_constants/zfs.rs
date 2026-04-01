// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// ZFS command names
    ///
    /// ZFS command name
    pub const ZFS: &str = "zfs";
    /// ZPOOL command name
    pub const ZPOOL: &str = "zpool";
    /// LIST command
    pub const LIST: &str = "list";
    /// CREATE command
    pub const CREATE: &str = "create";
    /// DESTROY command
    pub const DESTROY: &str = "destroy";
    /// SET command
    pub const SET: &str = "set";
    /// GET command
    pub const GET: &str = "get";
    /// SNAPSHOT command
    pub const SNAPSHOT: &str = "snapshot";
    /// STATUS command
    pub const STATUS: &str = "status";

    /// ZFS pool states
    ///
    /// Pool is online and functioning normally
    pub const ONLINE: &str = "ONLINE";
    /// Pool is degraded but still operational
    pub const DEGRADED: &str = "DEGRADED";
    /// Pool has faulted and is not operational
    pub const FAULTED: &str = "FAULTED";
    /// Pool is offline
    pub const OFFLINE: &str = "OFFLINE";
    /// Pool is unavailable
    pub const UNAVAIL: &str = "UNAVAIL";
    /// Pool device has been removed
    pub const REMOVED: &str = "REMOVED";

    /// ZFS properties
    ///
    /// Property selector for all properties
    pub const PROPERTY_ALL: &str = "all";
    /// Metadata property
    pub const PROPERTY_METADATA: &str = "metadata";
    /// Property enabled/on value
    pub const PROPERTY_ON: &str = "on";
    /// Property disabled/off value
    pub const PROPERTY_OFF: &str = "off";
    /// Standard property value
    pub const PROPERTY_STANDARD: &str = "standard";
    /// Disabled property value
    pub const PROPERTY_DISABLED: &str = "disabled";

    /// ZFS record sizes
    ///
    /// Record size of 64 KB
    pub const RECORDSIZE_64K: &str = "64K";
    /// Record size of 128 KB
    pub const RECORDSIZE_128K: &str = "128K";
    /// Record size of 1 MB
    pub const RECORDSIZE_1M: &str = "1M";

    /// ZFS file system types
    ///
    /// ZFS file system type
    pub const FSTYPE_ZFS: &str = "zfs";
    /// EXT4 file system type
    pub const FSTYPE_EXT4: &str = "ext4";
    /// Temporary file system type
    pub const FSTYPE_TMPFS: &str = "tmpfs";
    /// Device temporary file system type
    pub const FSTYPE_DEVTMPFS: &str = "devtmpfs";

    /// ZFS mount points
    /// Root mount point
    pub const MOUNTPOINT_ROOT: &str = "/";
    /// Boot mount point
    pub const MOUNTPOINT_BOOT: &str = "/boot";
