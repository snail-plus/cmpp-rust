
pub const DEFAULT_LISTENING_ADDR: &'static str = "0.0.0.0:8888";

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub addr: String,
    pub rate: usize,
}


impl Default for Config {
    fn default() -> Self {
        Config{
            addr: DEFAULT_LISTENING_ADDR.to_owned(),
            rate: 6000,
        }
    }
}
