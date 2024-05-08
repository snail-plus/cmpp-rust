
pub const DEFAULT_LISTENING_ADDR: &'static str = "0.0.0.0:8888";

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub addr: String,
    pub target_addr: String,
}


impl Default for Config {
    fn default() -> Config {
        Config{
            addr: DEFAULT_LISTENING_ADDR.to_owned(),
            target_addr: TARGET_ADDR.to_owned(),
        }
    }
}
