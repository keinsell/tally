pub const APP_ID: &str = match option_env!("APP_ID") {
    Some(id) => id,
    None => "com.keinsell.Tally",
};
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RESOURCE_BASE: &str = "/com/keinsell/Tally/";
pub const COUNTER_RESOURCES: &[u8] = include_bytes!(env!("COUNTER_RESOURCES_FILE"));
