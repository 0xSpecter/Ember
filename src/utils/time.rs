use std::sync::OnceLock;
use std::time::Instant;

pub fn millis() -> u128 {
    static START: OnceLock<Instant> = OnceLock::new();
    START.get_or_init(Instant::now).elapsed().as_millis()
}
