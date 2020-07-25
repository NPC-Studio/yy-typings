use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};
use smart_default::SmartDefault;
use std::{convert::TryFrom, fmt};

/// Describes the current event type. Users can make most events freely, though
/// special care should be taken that `Alarm`'s .0 field is less than `ALARM_MAX`,
/// and the same for the `OtherEvent`'s usize wrappers.
///
/// As of `0.1.3`, these typings do not include most of the input interrupts, like
/// KeyDown(x). They will eventually, when I develop the mental stamina to type them.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Copy, Clone)]
pub enum EventType {
    #[default]
    Create,
    Destroy,
    Cleanup,

    Step(Stage),

    Alarm(usize),

    Draw(DrawEvent),
    Collision,
    Other(OtherEvent),
    Async(AsyncEvent),
}

impl EventType {
    /// The maximum number of alarms which are available in the Gms2 IDE.
    pub const ALARM_MAX: usize = 11;

    /// Gets the filename for a given event with its requisite base. We return in this format
    /// to reduce allocating a string per call, as this filename will likely become allocated
    /// on some path in the future.
    ///
    /// ```rs
    /// let (base_name, numeric_id) = EventType::Create;
    /// assert_eq!(base_name, "Create_");
    /// assert_eq!(numeric_id, 0);
    /// ```
    ///
    /// If `event_type != EventType::Unknown`, then this function will *always* return a `Some`.
    /// `None` is reserved only for `EventType::Unknown`.
    pub fn filename(&self) -> Option<(&'static str, usize)> {
        let name = match self {
            EventType::Create => "Create_",
            EventType::Destroy => "Destroy_",
            EventType::Cleanup => "CleanUp_",
            EventType::Step(_) => "Step_",
            EventType::Alarm(_) => "Alarm_",
            EventType::Draw(_) => "Draw_",
            EventType::Collision => "Collision_",
            EventType::Other(_) => "Other_",
            EventType::Async(_) => "Other_",
        };

        let number = EventIntermediary::from(*self).event_num;

        Some((name, number))
    }

    /// Parses a given filename and number into an `EventType`, if valid.
    pub fn parse_filename(value: &str, event_num: usize) -> Option<EventType> {
        let event_type = match value {
            "Create_" => 0,
            "Destroy_" => 1,
            "CleanUp_" => 12,
            "Step_" => 3,
            "Alarm_" => 2,
            "Draw_" => 8,
            "Collision_" => 4,
            "Other_" => 7,
            _ => return None,
        };

        EventType::try_from(EventIntermediary { event_type, event_num}).ok()
    }
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

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Copy, Clone)]
pub enum OtherEvent {
    #[default]
    OutsideRoom,
    IntersectBoundary,
    OutsideView(usize),
    IntersectView(usize),
    GameStart,
    GameEnd,
    RoomStart,
    RoomEnd,
    AnimationEnd,
    AnimationUpdate,
    AnimationEvent,
    PathEnded,
    UserEvent(usize),
    BroadcastMessage,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Copy, Clone)]
pub enum AsyncEvent {
    #[default]
    AudioPlayback,
    AudioRecording,
    Cloud, // like FF7
    Dialog,
    Http,
    InAppPurchase,
    ImageLoaded,
    Networking,
    PushNotification,
    SaveLoad,
    Social,
    Steam,
    System,
}

impl OtherEvent {
    pub const OUTSIDE_VIEW_BASE: usize = 40;
    pub const OUTSIDE_VIEW_MAX: usize = 7;

    pub const INTERSECT_VIEW_BASE: usize = 50;

    pub const USER_EVENT_BASE: usize = 10;
    pub const USER_EVENT_MAX: usize = 15;
}

/// A simpler, less idiomatic and less understandable, but more direct, representation of
/// Gms2 event types and numbers. We use this internally in the serde of the higher level
/// `EventType` enum, which is also given.
///
/// This struct is made public largely so non-Rust applications downstream can have an easier
/// interface to work with. Rust applications are encouraged to stick with the more idiomatic
/// and user-friendly `EventType`, which is far more type safe while being equally performant.
#[derive(
    Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Serialize, Deserialize, Copy, Clone,
)]
pub struct EventIntermediary {
    event_type: usize,
    event_num: usize,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum EventTypeConvertErrors {
    CannotFindEventNumber(usize),
    CannotFindEventType,
}

impl std::error::Error for EventTypeConvertErrors {}
impl fmt::Display for EventTypeConvertErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventTypeConvertErrors::CannotFindEventNumber(event_type) => {
                write!(f, "invalid event_number for event type {}", event_type)
            }
            EventTypeConvertErrors::CannotFindEventType => write!(f, "invalid event_type"),
        }
    }
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
            EventType::Collision => EventIntermediary {
                event_type: 4,
                event_num: 0,
            },
            EventType::Other(other_event) => EventIntermediary {
                event_type: 7,
                event_num: match other_event {
                    OtherEvent::OutsideRoom => 0,
                    OtherEvent::IntersectBoundary => 1,
                    OtherEvent::OutsideView(val) => OtherEvent::OUTSIDE_VIEW_BASE + val,
                    OtherEvent::IntersectView(val) => OtherEvent::INTERSECT_VIEW_BASE + val,
                    OtherEvent::GameStart => 2,
                    OtherEvent::GameEnd => 3,
                    OtherEvent::RoomStart => 4,
                    OtherEvent::RoomEnd => 5,
                    OtherEvent::AnimationEnd => 7,
                    OtherEvent::AnimationUpdate => 58,
                    OtherEvent::AnimationEvent => 59,
                    OtherEvent::PathEnded => 8,
                    OtherEvent::UserEvent(ev_num) => OtherEvent::USER_EVENT_BASE + ev_num,
                    OtherEvent::BroadcastMessage => 76,
                },
            },
            EventType::Async(async_event) => EventIntermediary {
                event_type: 7,
                event_num: match async_event {
                    AsyncEvent::AudioPlayback => 74,
                    AsyncEvent::AudioRecording => 73,
                    AsyncEvent::Cloud => 67,
                    AsyncEvent::Dialog => 63,
                    AsyncEvent::Http => 62,
                    AsyncEvent::InAppPurchase => 66,
                    AsyncEvent::ImageLoaded => 60,
                    AsyncEvent::Networking => 68,
                    AsyncEvent::PushNotification => 71,
                    AsyncEvent::SaveLoad => 72,
                    AsyncEvent::Social => 70,
                    AsyncEvent::Steam => 69, // nice
                    AsyncEvent::System => 75,
                },
            },
        }
    }
}

impl TryFrom<EventIntermediary> for EventType {
    type Error = EventTypeConvertErrors;
    fn try_from(o: EventIntermediary) -> Result<Self, Self::Error> {
        const USER_EVENT_MAX_ABS: usize = OtherEvent::USER_EVENT_BASE + OtherEvent::USER_EVENT_MAX;
        const OUTSIDE_VIEW_MAX: usize =
            OtherEvent::OUTSIDE_VIEW_BASE + OtherEvent::OUTSIDE_VIEW_MAX;
        const OUTSIDE_INTERSECT_MAX: usize =
            OtherEvent::INTERSECT_VIEW_BASE + OtherEvent::OUTSIDE_VIEW_MAX;
        let output = match o.event_type {
            // lifetime events
            0 if o.event_num == 0 => EventType::Create,
            1 if o.event_num == 0 => EventType::Destroy,
            12 if o.event_num == 0 => EventType::Cleanup,

            // step
            3 => match o.event_num {
                0 => EventType::Step(Stage::Main),
                1 => EventType::Step(Stage::Begin),
                2 => EventType::Step(Stage::End),
                _ => return Err(EventTypeConvertErrors::CannotFindEventNumber(3)),
            },

            2 if o.event_num <= EventType::ALARM_MAX => EventType::Alarm(o.event_num),

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
                _ => return Err(EventTypeConvertErrors::CannotFindEventNumber(8)),
            },

            4 if o.event_num == 0 => EventType::Collision,

            7 => match o.event_num {
                // OTHER EVENTS
                0 => EventType::Other(OtherEvent::OutsideRoom),
                1 => EventType::Other(OtherEvent::IntersectBoundary),
                val @ OtherEvent::OUTSIDE_VIEW_BASE..=OUTSIDE_VIEW_MAX => {
                    EventType::Other(OtherEvent::OutsideView(val - OtherEvent::OUTSIDE_VIEW_BASE))
                }
                val @ OtherEvent::INTERSECT_VIEW_BASE..=OUTSIDE_INTERSECT_MAX => EventType::Other(
                    OtherEvent::IntersectView(val - OtherEvent::INTERSECT_VIEW_BASE),
                ),
                2 => EventType::Other(OtherEvent::GameStart),
                3 => EventType::Other(OtherEvent::GameEnd),
                4 => EventType::Other(OtherEvent::RoomStart),
                5 => EventType::Other(OtherEvent::RoomEnd),

                7 => EventType::Other(OtherEvent::AnimationEnd),
                58 => EventType::Other(OtherEvent::AnimationUpdate),
                59 => EventType::Other(OtherEvent::AnimationEvent),
                8 => EventType::Other(OtherEvent::PathEnded),
                val @ OtherEvent::USER_EVENT_BASE..=USER_EVENT_MAX_ABS => {
                    EventType::Other(OtherEvent::UserEvent(val - OtherEvent::USER_EVENT_BASE))
                }
                76 => EventType::Other(OtherEvent::BroadcastMessage),

                // ASYNC EVENTS
                74 => EventType::Async(AsyncEvent::AudioPlayback),
                73 => EventType::Async(AsyncEvent::AudioRecording),
                67 => EventType::Async(AsyncEvent::Cloud),
                63 => EventType::Async(AsyncEvent::Dialog),
                62 => EventType::Async(AsyncEvent::Http),
                66 => EventType::Async(AsyncEvent::InAppPurchase),
                60 => EventType::Async(AsyncEvent::ImageLoaded),
                68 => EventType::Async(AsyncEvent::Networking),
                71 => EventType::Async(AsyncEvent::PushNotification),
                72 => EventType::Async(AsyncEvent::SaveLoad),
                70 => EventType::Async(AsyncEvent::Social),
                69 => EventType::Async(AsyncEvent::Steam),
                75 => EventType::Async(AsyncEvent::System),

                _ => return Err(EventTypeConvertErrors::CannotFindEventNumber(7)),
            },
            _ => return Err(EventTypeConvertErrors::CannotFindEventType),
        };

        Ok(output)
    }
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

        EventType::try_from(event_intermediary).map_err(Error::custom)
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<EventType, D::Error>
    where
        D: Deserializer<'de>,
    {
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
        let event_type: Result<EventType, _> = serde_json::from_str(event_type_str);
        assert!(event_type.is_err());
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

        harness(EventType::Collision);

        harness(EventType::Other(OtherEvent::OutsideRoom));
        harness(EventType::Other(OtherEvent::IntersectBoundary));
        for i in 0..=OtherEvent::OUTSIDE_VIEW_MAX {
            harness(EventType::Other(OtherEvent::OutsideView(i)));
            harness(EventType::Other(OtherEvent::IntersectView(i)));
        }
        harness(EventType::Other(OtherEvent::GameStart));
        harness(EventType::Other(OtherEvent::GameEnd));
        harness(EventType::Other(OtherEvent::RoomStart));
        harness(EventType::Other(OtherEvent::RoomEnd));
        harness(EventType::Other(OtherEvent::AnimationEnd));
        harness(EventType::Other(OtherEvent::AnimationUpdate));
        harness(EventType::Other(OtherEvent::AnimationEvent));
        harness(EventType::Other(OtherEvent::PathEnded));
        for i in 0..=OtherEvent::USER_EVENT_MAX {
            harness(EventType::Other(OtherEvent::UserEvent(i)));
        }
        harness(EventType::Other(OtherEvent::BroadcastMessage));

        harness(EventType::Async(AsyncEvent::AudioRecording));
        harness(EventType::Async(AsyncEvent::AudioRecording));
        harness(EventType::Async(AsyncEvent::Cloud));
        harness(EventType::Async(AsyncEvent::Dialog));
        harness(EventType::Async(AsyncEvent::Http));
        harness(EventType::Async(AsyncEvent::InAppPurchase));
        harness(EventType::Async(AsyncEvent::ImageLoaded));
        harness(EventType::Async(AsyncEvent::Networking));
        harness(EventType::Async(AsyncEvent::PushNotification));
        harness(EventType::Async(AsyncEvent::SaveLoad));
        harness(EventType::Async(AsyncEvent::Social));
        harness(EventType::Async(AsyncEvent::Steam));
        harness(EventType::Async(AsyncEvent::System));

        fn harness(val: EventType) {
            let output = EventType::try_from(EventIntermediary::from(val)).unwrap();

            assert_eq!(val, output);
        }
    }

    #[test]
    fn symmetry_from_event_intermediary() {
        for event_type in 0..100 {
            for event_num in 0..100 {
                let ei = EventIntermediary {
                    event_type,
                    event_num,
                };

                if let Ok(et) = EventType::try_from(ei) {
                    assert_eq!(EventIntermediary::from(et), ei);
                }
            }
        }
    }
}