pub const APP_ID: &str = match option_env!("APP_ID") {
    Some(id) => id,
    None => "dev.keinsell.Counter",
};
pub const VERSION: &str = match option_env!("VERSION") {
    Some(version) => version,
    None => "1.0.0",
};
pub const RESOURCE_BASE: &str = "/dev/keinsell/Counter/";
pub const COUNTER_RESOURCES: &[u8] = include_bytes!(env!("COUNTER_RESOURCES_FILE"));
