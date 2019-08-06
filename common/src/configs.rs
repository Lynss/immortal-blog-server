use std::time::Duration;

pub const BACKEND_LOG_CONFIG: &'static str = "backend/configs/log4rs.yaml";
pub const EXPIRE_TIME: i64 = 60 * 60 * 24 * 30;

pub const ACTIVATED_EMAIL_EXPIRE_TIME: i64 = 60 * 60 * 24 * 1;
pub const USER_PREFIX_KEY: &'static str = "immortal:user";

/// How often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub const EVENTSOURCE: &'static str = "immortal:user:eventsource";
pub const EVENT_QUEUE: &'static str = "immortal:user:queue";
