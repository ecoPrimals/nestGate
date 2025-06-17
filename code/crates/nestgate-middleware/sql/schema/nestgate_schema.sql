-- Schema for TrueNAS ZFS Integration: NestGate Tiering, AI Workload Detection, and Telemetry

-- Tier Configuration
CREATE TABLE IF NOT EXISTS nestgate_tiering (
    id SERIAL PRIMARY KEY,
    tier_dataset VARCHAR(255) NOT NULL UNIQUE,
    tier_tier VARCHAR(50) NOT NULL CHECK (tier_tier IN ('hot', 'warm', 'cold', 'cache')),
    tier_properties TEXT NOT NULL,
    tier_workload_type VARCHAR(50) CHECK (tier_workload_type IN ('training', 'inference', 'checkpointing', NULL)),
    tier_created BIGINT NOT NULL,
    tier_updated BIGINT NOT NULL
);

-- AI Workload Detection Configuration
CREATE TABLE IF NOT EXISTS nestgate_aiworkload (
    id SERIAL PRIMARY KEY,
    ai_dataset VARCHAR(255) NOT NULL UNIQUE,
    ai_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    ai_sampling_period INTEGER NOT NULL DEFAULT 3600,
    ai_min_samples INTEGER NOT NULL DEFAULT 10,
    ai_auto_tune BOOLEAN NOT NULL DEFAULT FALSE,
    ai_last_detected BIGINT,
    ai_workload_type VARCHAR(50) CHECK (ai_workload_type IN ('training', 'inference', 'checkpointing', NULL)),
    ai_created BIGINT NOT NULL,
    ai_updated BIGINT NOT NULL
);

-- Tier Operations (Migration Jobs)
CREATE TABLE IF NOT EXISTS nestgate_tierops (
    id SERIAL PRIMARY KEY,
    op_source VARCHAR(255) NOT NULL,
    op_target VARCHAR(255) NOT NULL,
    op_tier VARCHAR(50) NOT NULL CHECK (op_tier IN ('hot', 'warm', 'cold', 'cache')),
    op_time_started BIGINT NOT NULL,
    op_time_completed BIGINT,
    op_status VARCHAR(50) NOT NULL CHECK (op_status IN ('in_progress', 'completed', 'failed')),
    op_error_msg TEXT
);

-- Tier Telemetry
CREATE TABLE IF NOT EXISTS nestgate_telemetry (
    id SERIAL PRIMARY KEY,
    tel_dataset VARCHAR(255) NOT NULL,
    tel_timestamp BIGINT NOT NULL,
    tel_read_ops INTEGER NOT NULL DEFAULT 0,
    tel_write_ops INTEGER NOT NULL DEFAULT 0,
    tel_read_bytes BIGINT NOT NULL DEFAULT 0,
    tel_write_bytes BIGINT NOT NULL DEFAULT 0,
    tel_read_latency REAL,
    tel_write_latency REAL,
    tel_access_pattern VARCHAR(50) CHECK (tel_access_pattern IN (
        'sequential_read', 'sequential_write', 'random_read', 'random_write', 'mixed', 'unknown'
    ))
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_tiering_dataset ON nestgate_tiering(tier_dataset);
CREATE INDEX IF NOT EXISTS idx_aiworkload_dataset ON nestgate_aiworkload(ai_dataset);
CREATE INDEX IF NOT EXISTS idx_telemetry_dataset_timestamp ON nestgate_telemetry(tel_dataset, tel_timestamp);
CREATE INDEX IF NOT EXISTS idx_telemetry_timestamp ON nestgate_telemetry(tel_timestamp);
CREATE INDEX IF NOT EXISTS idx_tierops_source ON nestgate_tierops(op_source);
CREATE INDEX IF NOT EXISTS idx_tierops_status ON nestgate_tierops(op_status);

-- Add default tier properties
INSERT INTO nestgate_tiering (
    tier_dataset, 
    tier_tier, 
    tier_properties, 
    tier_created, 
    tier_updated
) VALUES (
    'default_hot',
    'hot',
    '{"recordsize":"128K","compression":"lz4","primarycache":"all","secondarycache":"all","sync":"standard","logbias":"throughput","atime":"off"}',
    EXTRACT(EPOCH FROM NOW()),
    EXTRACT(EPOCH FROM NOW())
) ON CONFLICT DO NOTHING;

INSERT INTO nestgate_tiering (
    tier_dataset, 
    tier_tier, 
    tier_properties, 
    tier_created, 
    tier_updated
) VALUES (
    'default_warm',
    'warm',
    '{"recordsize":"1M","compression":"zstd","primarycache":"metadata","secondarycache":"all","sync":"standard","logbias":"latency","atime":"off"}',
    EXTRACT(EPOCH FROM NOW()),
    EXTRACT(EPOCH FROM NOW())
) ON CONFLICT DO NOTHING;

INSERT INTO nestgate_tiering (
    tier_dataset, 
    tier_tier, 
    tier_properties, 
    tier_created, 
    tier_updated
) VALUES (
    'default_cold',
    'cold',
    '{"recordsize":"1M","compression":"zstd-19","primarycache":"metadata","secondarycache":"metadata","sync":"standard","logbias":"throughput","atime":"off"}',
    EXTRACT(EPOCH FROM NOW()),
    EXTRACT(EPOCH FROM NOW())
) ON CONFLICT DO NOTHING;

INSERT INTO nestgate_tiering (
    tier_dataset, 
    tier_tier, 
    tier_properties, 
    tier_created, 
    tier_updated
) VALUES (
    'default_cache',
    'cache',
    '{"recordsize":"128K","compression":"lz4","primarycache":"all","secondarycache":"all","sync":"always","logbias":"latency","atime":"on"}',
    EXTRACT(EPOCH FROM NOW()),
    EXTRACT(EPOCH FROM NOW())
) ON CONFLICT DO NOTHING;

-- Function to periodically clean up old telemetry data
-- Keeps most recent 1000 records per dataset and records from last 30 days
CREATE OR REPLACE FUNCTION cleanup_telemetry() RETURNS void AS $$
DECLARE
    dataset_name VARCHAR;
    max_age_timestamp BIGINT;
    record_count INTEGER;
    records_to_delete INTEGER;
BEGIN
    -- Set maximum age to 30 days ago
    max_age_timestamp := EXTRACT(EPOCH FROM (NOW() - INTERVAL '30 days'));

    -- Delete records older than max_age
    DELETE FROM nestgate_telemetry WHERE tel_timestamp < max_age_timestamp;

    -- For each dataset, limit to 1000 records
    FOR dataset_name IN 
        SELECT DISTINCT tel_dataset FROM nestgate_telemetry 
    LOOP
        SELECT COUNT(*) INTO record_count FROM nestgate_telemetry WHERE tel_dataset = dataset_name;
        
        IF record_count > 1000 THEN
            records_to_delete := record_count - 1000;
            
            DELETE FROM nestgate_telemetry 
            WHERE id IN (
                SELECT id FROM nestgate_telemetry 
                WHERE tel_dataset = dataset_name 
                ORDER BY tel_timestamp ASC 
                LIMIT records_to_delete
            );
        END IF;
    END LOOP;
END;
$$ LANGUAGE plpgsql; 