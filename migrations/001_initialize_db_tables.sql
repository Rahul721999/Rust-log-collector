-- # Add System-Logs Table
CREATE TABLE IF NOT EXISTS system_logs
(
    timestamp    String,
    log_id       String,
    log_level    String,
    log_type     String,
    scope        String,
    module       String,
    user_id      String DEFAULT '',
    user_name    String DEFAULT '',
    event_id     String,
    log_msg      String,
    action       String,
    metadata     String
)
ENGINE = MergeTree()
ORDER BY (timestamp);