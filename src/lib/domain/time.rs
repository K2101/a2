use super::{DomainError, Result};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, UtcOffset};

// bangkok +7 hours
const OFFSET: i32 = 7 * 3600;

#[derive(Clone, Debug, Display, Deserialize, Serialize)]
pub struct Time(i64);
// unix timestamp will overflow i64 with in 239 years from 2022

impl Time {
    pub fn new(timestamp: i64) -> Self {
        Self(timestamp)
    }
    pub fn now() -> Self {
        // return unix nano
        let now = OffsetDateTime::now_utc();
        let now_nano = now.unix_timestamp_nanos() as i64;

        Self(now_nano)
    }

    pub fn now_unix_seconds() -> Self {
        // return unix nano
        let now = OffsetDateTime::now_utc();
        let now_nano = now.unix_timestamp();

        Self(now_nano)
    }

    pub fn into_inner(self) -> i64 {
        self.0
    }

    pub fn to_utc_seconds(time: i64) -> Result<i64> {
        let time = OffsetDateTime::from_unix_timestamp_nanos(time as i128);
        match time {
            Ok(time) => Ok(time.unix_timestamp()),
            Err(err) => Err(DomainError::ConvertUnixNanoToSecondsError(err.to_string())),
        }
    }

    pub fn get_local_reading_time(&self) -> Result<String> {
        // return 2022-11-13 17:41:37
        let reading = match OffsetDateTime::from_unix_timestamp_nanos(self.0 as i128) {
            Ok(now) => now,
            Err(err) => return Err(DomainError::GetUnixNanoTimeError(err.to_string())),
        };

        let offset = UtcOffset::from_whole_seconds(OFFSET).expect("invalid offset");
        let local_reading = reading.to_offset(offset);
        let local_string = format!("{}", local_reading);
        let date_char: Vec<char> = local_string.chars().collect();
        let mut datetime = String::from("");

        for i in 0..date_char.len() - 18 {
            datetime.push(date_char[i])
        }

        Ok(datetime)
    }

    pub fn get_local_full_time(&self) -> Result<String> {
        // 2022-11-13 17:39:21.9898028 +07:00:00
        let full_reading = match OffsetDateTime::from_unix_timestamp_nanos(self.0 as i128) {
            Ok(now) => now,
            Err(err) => return Err(DomainError::GetUnixNanoTimeError(err.to_string())),
        };
        let offset = UtcOffset::from_whole_seconds(OFFSET).expect("invalid offset");
        let full_datetime = format!("{}", full_reading.to_offset(offset));

        Ok(full_datetime)
    }
}
