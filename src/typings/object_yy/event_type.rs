use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};
use smart_default::SmartDefault;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Copy, Clone)]
pub enum EventType {
    #[default]
    Create,
    Destroy,
    Cleanup,

    Step(Stage),

    Alarm(usize),

    Draw(DrawEvent),
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Copy, Clone)]
pub enum Stage {
    #[default]
    Main,
    Begin,
    End,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Copy, Clone)]
pub enum DrawEvent {
    #[default]
    Draw(Stage),
    DrawGui(Stage),
    PreDraw,
    PostDraw,
    WindowResize,
}

impl From<EventType> for EventIntermediary {
    fn from(o: EventType) -> Self {
        match o {
            EventType::Create => EventIntermediary {
                event_num: 0,
                event_type: 0,
            },
            EventType::Destroy => EventIntermediary {
                event_type: 1,
                event_num: 0,
            },
            EventType::Cleanup => EventIntermediary {
                event_type: 12,
                event_num: 0,
            },
            EventType::Step(stage) => match stage {
                Stage::Main => EventIntermediary {
                    event_type: 3,
                    event_num: 0,
                },
                Stage::Begin => EventIntermediary {
                    event_type: 3,
                    event_num: 1,
                },
                Stage::End => EventIntermediary {
                    event_type: 3,
                    event_num: 2,
                },
            },
            EventType::Alarm(alarm_number) => EventIntermediary {
                event_type: 2,
                event_num: alarm_number,
            },
            EventType::Draw(draw_event) => match draw_event {
                DrawEvent::Draw(stage) => match stage {
                    Stage::Main => EventIntermediary {
                        event_type: 8,
                        event_num: 0,
                    },
                    Stage::Begin => EventIntermediary {
                        event_type: 8,
                        event_num: 72,
                    },
                    Stage::End => EventIntermediary {
                        event_type: 8,
                        event_num: 73,
                    },
                },
                DrawEvent::DrawGui(stage) => match stage {
                    Stage::Main => EventIntermediary {
                        event_type: 8,
                        event_num: 64,
                    },
                    Stage::Begin => EventIntermediary {
                        event_type: 8,
                        event_num: 74,
                    },
                    Stage::End => EventIntermediary {
                        event_type: 8,
                        event_num: 75,
                    },
                },
                DrawEvent::PreDraw => EventIntermediary {
                    event_type: 8,
                    event_num: 76,
                },
                DrawEvent::PostDraw => EventIntermediary {
                    event_type: 8,
                    event_num: 77,
                },
                DrawEvent::WindowResize => EventIntermediary {
                    event_type: 8,
                    event_num: 65,
                },
            },
            EventType::Unknown => EventIntermediary {
                event_num: 0,
                event_type: 0,
            },
        }
    }
}

impl From<EventIntermediary> for EventType {
    fn from(o: EventIntermediary) -> Self {
        match o.event_type {
            // lifetime events
            0 if o.event_num == 0 => EventType::Create,
            1 if o.event_num == 0 => EventType::Destroy,
            12 if o.event_num == 0 => EventType::Cleanup,

            // step
            3 => match o.event_num {
                0 => EventType::Step(Stage::Main),
                1 => EventType::Step(Stage::Begin),
                2 => EventType::Step(Stage::End),
                _ => EventType::Unknown,
            },

            2 if o.event_num < 12 => EventType::Alarm(o.event_num),

            8 => match o.event_num {
                0 => EventType::Draw(DrawEvent::Draw(Stage::Main)),
                72 => EventType::Draw(DrawEvent::Draw(Stage::Begin)),
                73 => EventType::Draw(DrawEvent::Draw(Stage::End)),
                64 => EventType::Draw(DrawEvent::DrawGui(Stage::Main)),
                74 => EventType::Draw(DrawEvent::DrawGui(Stage::Begin)),
                75 => EventType::Draw(DrawEvent::DrawGui(Stage::End)),
                76 => EventType::Draw(DrawEvent::PreDraw),
                77 => EventType::Draw(DrawEvent::PostDraw),
                65 => EventType::Draw(DrawEvent::WindowResize),
                _ => EventType::Unknown,
            },

            _ => EventType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Serialize, Deserialize)]
struct EventIntermediary {
    event_type: usize,
    event_num: usize,
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

#[derive(Debug, Serialize, Deserialize)]
enum Field {
    #[serde(rename = "event_num")]
    Number,
    #[serde(rename = "event_type")]
    Type,
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<EventType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, MapAccess, Visitor};
        struct DeserializerVisitor;

        impl<'de> Visitor<'de> for DeserializerVisitor {
            type Value = EventType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(r#"a value of "event_num""#)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut event_number = None;
                let mut event_type = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Number => {
                            if event_number.is_some() {
                                return Err(Error::duplicate_field("event_number"));
                            }
                            event_number = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if event_type.is_some() {
                                return Err(Error::duplicate_field("event_type"));
                            }
                            event_type = Some(map.next_value()?);
                        }
                    }
                }

                let event_intermediary = EventIntermediary {
                    event_type: event_type.ok_or_else(|| Error::missing_field("event_type"))?,
                    event_num: event_number.ok_or_else(|| Error::missing_field("event_number"))?,
                };

                Ok(EventType::from(event_intermediary))
            }
        }

        deserializer.deserialize_struct("EventType", &FIELD_NAMES, DeserializerVisitor)
    }
}

const FIELD_NAMES: [&str; 2] = ["event_num", "event_type"];

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
        let wrapper_str =
            r#"{"rev_padding":10,"event_num":0,"event_type":0,"ioop": "oop","padding":10}"#;
        let wrapper: Wrapper = serde_json::from_str(wrapper_str).unwrap();

        assert_eq!(
            wrapper,
            Wrapper {
                rev_padding: 10,
                event_type: EventType::Create,
                padding: 10
            }
        );

        let event_type_str = r#"{"event_num":0,"event_type":0}"#;
        let event_type: EventType = serde_json::from_str(event_type_str).unwrap();

        assert_eq!(event_type, EventType::Create);

        let event_type_str = r#"{"event_num":100,"event_type":1000}"#;
        let event_type: EventType = serde_json::from_str(event_type_str).unwrap();

        assert_eq!(event_type, EventType::Unknown);
    }

    #[test]
    fn symmetry() {
        harness(EventType::Create);
        harness(EventType::Destroy);
        harness(EventType::Cleanup);

        harness(EventType::Step(Stage::Main));
        harness(EventType::Step(Stage::Begin));
        harness(EventType::Step(Stage::End));

        for i in 0..=11 {
            harness(EventType::Alarm(i));
        }

        harness(EventType::Draw(DrawEvent::Draw(Stage::Main)));
        harness(EventType::Draw(DrawEvent::Draw(Stage::Begin)));
        harness(EventType::Draw(DrawEvent::Draw(Stage::End)));
        harness(EventType::Draw(DrawEvent::DrawGui(Stage::Main)));
        harness(EventType::Draw(DrawEvent::DrawGui(Stage::Begin)));
        harness(EventType::Draw(DrawEvent::DrawGui(Stage::End)));
        harness(EventType::Draw(DrawEvent::PreDraw));
        harness(EventType::Draw(DrawEvent::PostDraw));
        harness(EventType::Draw(DrawEvent::WindowResize));

        fn harness(val: EventType) {
            let output = EventType::from(EventIntermediary::from(val));

            assert_eq!(val, output);
        }
    }
}
