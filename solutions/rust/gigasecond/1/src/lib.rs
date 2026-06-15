use time::PrimitiveDateTime as DateTime;
use time::OffsetDateTime;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let mut sec = start.assume_utc().unix_timestamp();
    sec += 1_000_000_000;

    let offset = OffsetDateTime::from_unix_timestamp(sec).unwrap();
    DateTime::new(
        offset.date(),
        offset.time()
    )
}
