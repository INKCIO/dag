extern crate config;

use self::config::*;
use may::sync::RwLock;

pub const WS_PORT: u16 = 8080;
pub const COUNT_WITNESSES: usize = 12;
pub const MAJORITY_OF_WITNESSES: usize = 7;
pub const GENESIS_UNIT: &str = "rg1RzwKwnfRHjBojGol3gZaC5w7kR++rOR6O61JRsrQ=";
pub const VERSION: &str = "1.0";
pub const ALT: &str = "1";
pub const STALLED_TIMEOUT: usize = 10;
pub const MAX_MESSAGES_PER_UNIT: usize = 128;

pub const COUNT_MC_BALLS_FOR_PAID_WITNESSING: u32 = 100;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings
            .merge(File::with_name("settings.json"))
            .expect("failed to load config");
        settings
    });
}
