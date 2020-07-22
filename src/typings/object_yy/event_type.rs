use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};
use smart_default::SmartDefault;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Copy, Clone)]
pub enum EventType {
    #[default]
    Create,
    Unknown,
}

impl From<EventType> for EventIntermediary {
    fn from(o: EventType) -> Self {
        match o {
            EventType::Create => EventIntermediary {
                event_num: 0,
                event_type: 0,
            },
            // gah this feels terrible
            EventType::Unknown => EventIntermediary {
                event_num: 0,
                event_type: 0,
            },
        }
    }
}

impl From<EventIntermediary> for EventType {
    fn from(o: EventIntermediary) -> Self {
        match o.event_num {
            0 if o.event_type == 0 => EventType::Create,
            _ => EventType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Serialize, Deserialize)]
struct EventIntermediary {
    pub event_num: usize,
    pub event_type: usize,
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let val: EventIntermediary = (*self).into();
        let mut inter = serializer.serialize_struct("EventIntermediary", 2)?;
        inter.serialize_field("event_num", &val.event_num)?;
        inter.serialize_field("event_type", &val.event_type)?;
        inter.end()
    }
}

use serde::de::{Error, MapAccess, Visitor};
struct DeserializerVisitor;

impl<'de> Visitor<'de> for DeserializerVisitor {
    type Value = EventType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(r#"a value of "event_num""#)
    }

    fn visit_map<A>(self, v: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut event_number = None;
        let mut event_ = None;
        while let Some(key) = map.next_key()? {
            match key {
                Field::Secs => {
                    if secs.is_some() {
                        return Err(de::Error::duplicate_field("secs"));
                    }
                    secs = Some(map.next_value()?);
                }
                Field::Nanos => {
                    if nanos.is_some() {
                        return Err(de::Error::duplicate_field("nanos"));
                    }
                    nanos = Some(map.next_value()?);
                }
            }
        }
        let secs = secs.ok_or_else(|| de::Error::missing_field("secs"))?;
        let nanos = nanos.ok_or_else(|| de::Error::missing_field("nanos"))?;
        Ok(Duration::new(secs, nanos))

        unimplemented!();
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<EventType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DeserializerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
    struct Wrapper {
        pub rev_padding: usize,
        #[serde(flatten)]
        pub event_type: EventType,
        pub padding: usize,
    }

    #[test]
    fn basic_serialize() {
        let value = Wrapper {
            rev_padding: 10,
            event_type: EventType::Create,
            padding: 10,
        };
        let as_serde: String = serde_json::to_string(&value).unwrap();

        assert_eq!(
            as_serde,
            r#"{"rev_padding":10,"event_num":0,"event_type":0,"padding":10}"#
        )
    }

    #[test]
    fn basic_deserialize() {
        let wrapper_str = r#"{"rev_padding":10,"event_num":0,"event_type":0,"padding":10}"#;

        let wrapper: Wrapper = serde_json::from_str(wrapper_str).unwrap();

        assert_eq!(
            wrapper,
            Wrapper {
                rev_padding: 10,
                event_type: EventType::Create,
                padding: 10
            }
        );
    }
}
