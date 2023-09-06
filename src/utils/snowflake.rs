use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use snowflake::SnowflakeIdGenerator;
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sqlx::Type;

lazy_static! {
    // Fri, 01 Jan 2021 00:00:00 GMT
    static ref EPOCH: SystemTime = UNIX_EPOCH + Duration::from_millis(1609459200000);

    static ref GENERATOR: Mutex<SnowflakeIdGenerator> = Mutex::new(SnowflakeIdGenerator::with_epoch(0, 0, *EPOCH));
}

#[serde_as]
#[derive(Type, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[sqlx(transparent)]
pub struct Snowflake(#[serde_as(as = "serde_with::DisplayFromStr")] pub i64);

impl Snowflake {
    pub fn generate() -> Self {
        Self(GENERATOR.lock().unwrap().generate())
    }

    pub fn created_at_timestamp(&self) -> Duration {
        Duration::from_millis((**self >> 22) as u64) + EPOCH.duration_since(UNIX_EPOCH).unwrap()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.created_at_timestamp().as_secs() as i64, 0)
            .unwrap()
    }
}


impl Default for Snowflake {
    fn default() -> Self {
        Self::generate()
    }
}

impl ToString for Snowflake {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl std::ops::Deref for Snowflake {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl PgHasArrayType for Snowflake {
//     fn array_type_info() -> PgTypeInfo {
//         i64::array_type_info()
//     }
// 
//     fn array_compatible(_: &PgTypeInfo) -> bool {
//         true
//     }
// }

impl TryFrom<String> for Snowflake {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Snowflake(value.parse()?))
    }
}