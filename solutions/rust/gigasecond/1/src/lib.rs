extern crate chrono;
use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let dt = start + Duration::seconds(1000000000);
    //unimplemented!("What time is a gigasecond later than {}", start);
    dt
}
