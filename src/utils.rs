use chrono::{DateTime, Utc};
use serde::{Serializer, Serialize};

pub fn serialize_dt<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    dt.format("%d/%m/%Y %H:%M")
        .to_string()
        .serialize(serializer)
}

pub fn serialize_dt_option<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(dt) = dt {
        dt.format("%d/%m/%Y %H:%M")
            .to_string()
            .serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}
