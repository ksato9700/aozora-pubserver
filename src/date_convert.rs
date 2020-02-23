use chrono::SecondsFormat;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::ser::Serializer;
use std::collections::HashMap;
use std::fmt;

pub fn serialize<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(&datetime.to_rfc3339_opts(SecondsFormat::Secs, true))

  // let mut hmap = HashMap::<String, i64>::new();
  // hmap.insert("$numberLong".to_string(), datetime.timestamp_millis());
  // let mut tup = serializer.serialize_tuple(2)?;
  // tup.serialize_element("$date")?;
  // tup.serialize_element(&hmap)?;
  // tup.end()
}

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
      // println!("{:?}", entry);
      let timestamp = entry.1.get("$numberLong").unwrap();
      let sec = timestamp / 1000;
      let msec = timestamp % 1000;
      Ok(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(sec, msec as u32),
        Utc,
      ))
    }
  }

  let datetime = deserializer.deserialize_struct("", &[], UtcDateTimeVisitor);
  Ok(datetime.unwrap())
}
