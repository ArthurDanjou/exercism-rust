use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // Add 10 seconds to start
    start + Duration::seconds(1_000_000_000)
}
