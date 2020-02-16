use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::{MapAccess, Visitor};
use serde::Deserializer;
use std::collections::HashMap;
use std::fmt;

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    struct UtcDateTimeVisitor;

    impl<'de> Visitor<'de> for UtcDateTimeVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("format eror")
        }

        fn visit_map<A>(self, map: A) -> Result<DateTime<Utc>, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut map = map;
            let entry = map
                .next_entry::<String, HashMap<String, i64>>()
                .unwrap()
                .unwrap();
            let timestamp = entry.1.get("$numberLong").unwrap();
            let sec = timestamp / 1000;
            let msec = timestamp % 1000;
            Ok(DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(sec, msec as u32),
                Utc,
            ))
        }
    }

    // let s = String::deserialize(deserializer).unwrap();

    // let s = deserializer.deserialize_string(UtcDateTimeVisitor);

    let datetime = deserializer.deserialize_struct("", &[], UtcDateTimeVisitor);
    // Utc
    //   .datetime_from_str(&s, FORMAT)
    //   .map_err(serde::de::Error::custom)
    Ok(datetime.unwrap())
}
