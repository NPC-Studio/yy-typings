use super::{Gesture, MouseButtonCode, VirtualKeyCode};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt;

/// Describes the current event type. Users can make most events freely, though
/// special care should be taken that `Alarm`'s .0 field is less than
/// `ALARM_MAX`, and the same for the `OtherEvent`'s usize wrappers. To make
/// sure some event has been validly created, `is_valid` has been provided.
///
/// **Note: only serde_json serialization and deserialization is supported for
/// EventType.** Yaml, and other text / WYSIWYG data formats should be fine, but
/// Bincode and other binary sequences are unlikely to succesfully serialize
/// this. This is due to our use of serde's `flatten`, which runs afoul of this issue: https://github.com/servo/bincode/issues/245
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Copy, Clone)]
pub enum EventType {
    #[default]
    Create,

    Step(Stage),
    Draw(DrawEvent),

    Alarm(usize),
    Collision,

    Mouse(MouseEvent),

    KeyDown(VirtualKeyCode),
    KeyPress(VirtualKeyCode),
    KeyRelease(VirtualKeyCode),

    Gesture(GestureEvent),

    Other(OtherEvent),
    Async(AsyncEvent),

    Destroy,
    CleanUp,
}

impl EventType {
    /// The maximum number of alarms which are available in the Gms2 IDE.
    pub const ALARM_MAX: usize = 11;

    /// Returns the filename like it will appear in a file.
    pub fn filename(&self) -> String {
        use heck::ToPascalCase;

        let event_type_number: EventTypeNumber = self.into();
        let intermediary: EventIntermediary = self.into();

        let word = event_type_number.to_string().to_pascal_case();

        format!("{}_{}", word, intermediary.event_num)
    }

    /// Parses a given filename and number into an `EventType`, if valid.
    ///
    /// # Errors
    ///
    /// Errors if the `value` cannot be parsed into an EventTypeNumber or if the `event_num`
    /// is not valid for the given EventTypeNumber.
    pub fn parse_filename(
        value: &str,
        event_num: usize,
    ) -> Result<EventType, EventTypeConvertErrors> {
        let event_type: EventTypeNumber = value.parse()?;

        EventType::try_from(EventIntermediary {
            event_type: event_type as usize,
            event_num,
        })
    }

    /// Converts to a heuristic filename, suitable for humans.
    pub fn to_human_readable(&self) -> String {
        match self {
            EventType::Create => "create".to_string(),
            EventType::Destroy => "destroy".to_string(),
            EventType::CleanUp => "cleanup".to_string(),
            EventType::Step(stage) => stage.to_human_readable("step"),
            EventType::Alarm(n) => format!("alarm_{}", n),
            EventType::Draw(draw_event) => match draw_event {
                DrawEvent::Draw(stage) => stage.to_human_readable("draw"),
                DrawEvent::DrawGui(stage) => stage.to_human_readable("draw_gui"),
                DrawEvent::PreDraw => "pre_draw".to_string(),
                DrawEvent::PostDraw => "post_draw".to_string(),
                DrawEvent::WindowResize => "window_resize".to_string(),
            },
            EventType::Other(OtherEvent::RoomStart) => "room_start".to_string(),
            EventType::Other(OtherEvent::RoomEnd) => "room_end".to_string(),
            EventType::Other(OtherEvent::GameStart) => "game_start".to_string(),
            EventType::Other(OtherEvent::GameEnd) => "game_end".to_string(),
            EventType::Other(OtherEvent::AnimationEnd) => "animation_end".to_string(),
            EventType::Other(OtherEvent::UserEvent(n)) => format!("user_event_{}", n),
            EventType::Other(_)
            | EventType::Collision
            | EventType::Mouse(_)
            | EventType::KeyDown(_)
            | EventType::KeyPress(_)
            | EventType::KeyRelease(_)
            | EventType::Gesture(_)
            | EventType::Async(_) => {
                let event_type_number: EventTypeNumber = self.into();
                let intermediary: EventIntermediary = self.into();

                format!("{}_{}", event_type_number, intermediary.event_num)
            }
        }
    }

    /// A simple way to parse a value. It does a split on the string, which
    /// basically means it needs to follow the pattern `Create_0` and similar.
    /// 
    /// # Errors
    /// 
    /// Errors if the given input does not follow the pattern `x_y`, unless it's special cased,
    /// or if `x` is not a given EventTypeNumber or if `y` is not an appropriate event_num for the given
    /// EventTypeNumber.
    /// 
    /// In general, this function tries to "figure out" what EventType from a given input, and figuring out
    /// what input should be given is best done reading through the function.
    pub fn from_human_readable(value: &str) -> Result<EventType, EventTypeConvertErrors> {
        match value {
            "create" | "Create" => {
                return Ok(EventType::Create);
            }
            "game_start" | "GameStart" | "gameStart" => {
                return Ok(EventType::Other(OtherEvent::GameStart));
            }
            "game_end" | "GameEnd" | "gameEnd" => {
                return Ok(EventType::Other(OtherEvent::GameEnd));
            }
            "room_start" | "RoomStart" | "roomStart" => {
                return Ok(EventType::Other(OtherEvent::RoomStart));
            }
            "room_end" | "RoomEnd" | "roomEnd" => {
                return Ok(EventType::Other(OtherEvent::RoomEnd));
            }
            "animation_end" | "AnimationEnd" | "animationEnd" => {
                return Ok(EventType::Other(OtherEvent::AnimationEnd));
            }
            "draw_gui_begin" | "drawGuiBegin" | "DrawGuiBegin" => {
                return Ok(EventType::Draw(DrawEvent::DrawGui(Stage::Begin)));
            }
            "draw_gui" | "drawGui" | "DrawGui" => {
                return Ok(EventType::Draw(DrawEvent::DrawGui(Stage::Main)));
            }
            "draw_gui_end" | "drawGuiEnd" | "DrawGuiEnd" => {
                return Ok(EventType::Draw(DrawEvent::DrawGui(Stage::End)));
            }
            "pre_draw" | "preDraw" | "PreDraw" => {
                return Ok(EventType::Draw(DrawEvent::PreDraw));
            }
            "post_draw" | "postDraw" | "PostDraw" => {
                return Ok(EventType::Draw(DrawEvent::PostDraw));
            }
            "window_resize" | "windowResize" | "WindowResize" => {
                return Ok(EventType::Draw(DrawEvent::WindowResize));
            }
            _ => {}
        }

        let (name, value) = match value.rsplit_once('_') {
            Some(v) => v,
            None => (value, "0"),
        };

        let event_type: EventTypeNumber = name.parse()?;

        match value {
            "begin" => match event_type {
                EventTypeNumber::Step => Ok(EventType::Step(Stage::Begin)),
                EventTypeNumber::Draw => Ok(EventType::Draw(DrawEvent::Draw(Stage::Begin))),
                _ => Err(UnknownEventTypeNumber)?,
            },
            "end" => match event_type {
                EventTypeNumber::Step => Ok(EventType::Step(Stage::End)),
                EventTypeNumber::Draw => Ok(EventType::Draw(DrawEvent::Draw(Stage::End))),
                _ => Err(UnknownEventTypeNumber)?,
            },
            other => {
                let event_num: usize = other
                    .parse()
                    .map_err(|_| EventTypeConvertErrors::BadString)?;

                EventType::try_from(EventIntermediary {
                    event_type: event_type as usize,
                    event_num,
                })
            }
        }
    }

    pub fn is_valid(value: EventType) -> bool {
        EventType::try_from(EventIntermediary::from(value)).is_ok()
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Create => write!(f, "Create"),
            EventType::Destroy => write!(f, "Destroy"),
            EventType::CleanUp => write!(f, "CleanUp"),
            EventType::Step(stage) => write!(f, "{}", stage.display_with_before("Step")),
            EventType::Alarm(number) => write!(f, "Alarm {}", number),
            EventType::Draw(draw_stage) => write!(f, "{}", draw_stage),
            EventType::Collision => write!(f, "Collision"),
            EventType::Mouse(v) => write!(f, "{}", v),
            EventType::KeyDown(key) => write!(f, "Key Down - {}", key.as_ref()),
            EventType::KeyPress(key) => write!(f, "Key Press - {}", key.as_ref()),
            EventType::KeyRelease(key) => write!(f, "Key Up - {}", key.as_ref()),
            EventType::Gesture(gesture) => write!(f, "{}", gesture),
            EventType::Other(other) => write!(f, "{}", other),
            EventType::Async(async_ev) => write!(f, "{}", async_ev),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Copy, Clone)]
pub enum Stage {
    #[default]
    Main,
    Begin,
    End,
}

impl Stage {
    pub fn to_human_readable(&self, other: &str) -> String {
        match self {
            Stage::Main => other.to_string(),
            Stage::Begin => format!("{}_begin", other),
            Stage::End => format!("{}_end", other),
        }
    }

    pub fn display_with_before(&self, other: &str) -> String {
        match self {
            Stage::Main => other.to_string(),
            Stage::Begin => format!("Begin {}", other),
            Stage::End => format!("End {}", other),
        }
    }

    pub fn display_with_after(&self, other: &str) -> String {
        match self {
            Stage::Main => other.to_string(),
            Stage::Begin => format!("{} Begin", other),
            Stage::End => format!("{} End", other),
        }
    }
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

impl fmt::Display for DrawEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DrawEvent::Draw(stage) => write!(f, "{}", stage.display_with_after("Draw")),
            DrawEvent::DrawGui(stage) => write!(f, "{}", stage.display_with_after("Draw GUI")),
            DrawEvent::PreDraw => write!(f, "Pre-Draw"),
            DrawEvent::PostDraw => write!(f, "Post-Draw"),
            DrawEvent::WindowResize => write!(f, "Window Resize"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, SmartDefault, Copy, Clone)]
pub enum MouseEvent {
    #[default]
    Down(MouseButton),
    Pressed(MouseButton),
    Released(MouseButton),
    NoInput,
    MouseEnter,
    MouseExit,
    MouseWheelUp,
    MouseWheelDown,
}

impl MouseEvent {
    pub const DOWN_OFFSET: usize = 0;
    pub const PRESSED_OFFSET: usize = 4;
    pub const RELEASED_OFFSET: usize = 7;

    /// Tries to convert an `event_num` to a MouseEvent. This is for internal
    /// usage, but is made public to attempt to be a 100% pub facing crate.
    pub fn convert_to_input(mut value: usize) -> Option<MouseEvent> {
        let mut local_input = true;

        if value >= MouseButton::GLOBAL_OFFSET {
            local_input = false;
            value -= MouseButton::GLOBAL_OFFSET;
        }

        let output = match value {
            0..=2 => MouseEvent::Down(MouseButton {
                mb_code: num_traits::FromPrimitive::from_usize(value - Self::DOWN_OFFSET).unwrap(),
                local: local_input,
            }),
            3 => {
                if local_input {
                    MouseEvent::NoInput
                } else {
                    return None;
                }
            }
            4..=6 => MouseEvent::Pressed(MouseButton {
                mb_code: num_traits::FromPrimitive::from_usize(value - Self::PRESSED_OFFSET)
                    .unwrap(),
                local: local_input,
            }),
            _ => {
                if let Some(mouse_button_code) =
                    FromPrimitive::from_usize(value - Self::RELEASED_OFFSET)
                {
                    MouseEvent::Released(MouseButton {
                        mb_code: mouse_button_code,
                        local: local_input,
                    })
                } else {
                    return None;
                }
            }
        };

        Some(output)
    }
}

impl fmt::Display for MouseEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MouseEvent::Down(mb) => write!(f, "{} Down", mb),
            MouseEvent::Pressed(mb) => write!(f, "{} Pressed", mb),
            MouseEvent::Released(mb) => write!(f, "{} Released", mb),
            MouseEvent::NoInput => write!(f, "No Mouse Input"),
            MouseEvent::MouseEnter => write!(f, "Mouse Enter"),
            MouseEvent::MouseExit => write!(f, "Mouse Leave"),
            MouseEvent::MouseWheelUp => write!(f, "Mouse Wheel Up"),
            MouseEvent::MouseWheelDown => write!(f, "Mouse Wheel Down"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Copy, Clone)]
pub struct MouseButton {
    /// The mouse button code used for this input.
    pub mb_code: MouseButtonCode,

    /// Whether the input is a "global" input, or a "local" input. In the Gms2
    /// IDE, these are separated into different categories. "Local" events
    /// only file when the object itself is clicked on, while "global" can
    /// be fire whenever the input is held at all.
    pub local: bool,
}

impl MouseButton {
    /// The offset for the `event_num` if this mouse button is a global. We use
    /// this number internally for serialization/deserialization.
    pub const GLOBAL_OFFSET: usize = 50;

    /// Calculates the `event_num` offset for this `MouseButton`, largely for
    /// internal use in serialization and deserialization. We make this
    /// public as this library attempts to be 100% public.
    pub fn event_offset(&self) -> usize {
        self.mb_code as usize + if self.local { 0 } else { Self::GLOBAL_OFFSET }
    }
}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = match self.mb_code {
            MouseButtonCode::Left => "Left",
            MouseButtonCode::Right => "Right",
            MouseButtonCode::Middle => "Middle",
        };

        if self.local {
            write!(f, "{}", word)
        } else {
            write!(f, "Global {}", word)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Copy, Clone)]
pub struct GestureEvent {
    /// The type of gesture used.
    pub gesture: Gesture,
    /// Whether the input is a "global" input, or a "local" input. In the Gms2
    /// IDE, these are separated into different categories. "Local" events
    /// only file when the object itself is clicked on, while "global" can
    /// be fire whenever the input is held at all.
    pub local: bool,
}

impl GestureEvent {
    /// The offset for the `event_num` if this gesture is a global. We use this
    /// number internally for serialization/deserialization.
    pub const GLOBAL_OFFSET: usize = 64;

    /// Converts an `event_num`, if possible, into a Gesture.
    pub fn convert_to_input(mut value: usize) -> Option<GestureEvent> {
        let mut local = true;
        if value & Self::GLOBAL_OFFSET == Self::GLOBAL_OFFSET {
            value &= !Self::GLOBAL_OFFSET;
            local = false;
        }

        FromPrimitive::from_usize(value).map(|gesture| GestureEvent { gesture, local })
    }
}

impl fmt::Display for GestureEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = match self.gesture {
            Gesture::Tap => "Tap",
            Gesture::DoubleTap => "Double Tap",
            Gesture::DragStart => "Drag Start",
            Gesture::Dragging => "Dragging",
            Gesture::DragEnd => "Drag End",
            Gesture::Flick => "Flick",
            Gesture::PinchStart => "Pinch Start",
            Gesture::PinchIn => "Pinch In",
            Gesture::PinchOut => "Pinch Out",
            Gesture::PinchEnd => "Pinch End",
            Gesture::RotateStart => "Rotate Start",
            Gesture::Rotating => "Rotating",
            Gesture::RotateEnd => "Rotate End",
        };

        if self.local {
            write!(f, "{}", word)
        } else {
            write!(f, "Global {}", word)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Copy, Clone)]
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

impl fmt::Display for OtherEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OtherEvent::OutsideRoom => write!(f, "Outside Room"),
            OtherEvent::IntersectBoundary => write!(f, "Intersect Boundary"),
            OtherEvent::OutsideView(view) => write!(f, "Outside View {}", view),
            OtherEvent::IntersectView(view) => write!(f, "Intersect View {} Boundary", view),
            OtherEvent::GameStart => write!(f, "Game Start"),
            OtherEvent::GameEnd => write!(f, "Game End"),
            OtherEvent::RoomStart => write!(f, "Room Start"),
            OtherEvent::RoomEnd => write!(f, "Room End"),
            OtherEvent::AnimationEnd => write!(f, "Animation End"),
            OtherEvent::AnimationUpdate => write!(f, "Animation Update"),
            OtherEvent::AnimationEvent => write!(f, "Animation Event"),
            OtherEvent::PathEnded => write!(f, "Path Ended"),
            OtherEvent::UserEvent(event) => write!(f, "User Event {}", event),
            OtherEvent::BroadcastMessage => write!(f, "Broadcast Message"),
        }
    }
}

impl OtherEvent {
    pub const OUTSIDE_VIEW_BASE: usize = 40;
    pub const OUTSIDE_VIEW_MAX: usize = 7;

    pub const INTERSECT_VIEW_BASE: usize = 50;

    pub const USER_EVENT_BASE: usize = 10;
    pub const USER_EVENT_MAX: usize = 15;
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Copy, Clone)]
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

impl fmt::Display for AsyncEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = match self {
            AsyncEvent::AudioPlayback => "Audio Playback",
            AsyncEvent::AudioRecording => "Audio Recording",
            AsyncEvent::Cloud => "Cloud",
            AsyncEvent::Dialog => "Dialog",
            AsyncEvent::Http => "Http",
            AsyncEvent::InAppPurchase => "In-App Purchase",
            AsyncEvent::ImageLoaded => "Image Loaded",
            AsyncEvent::Networking => "Networking",
            AsyncEvent::PushNotification => "Push Notification",
            AsyncEvent::SaveLoad => "Save/Load",
            AsyncEvent::Social => "Social",
            AsyncEvent::Steam => "Steam",
            AsyncEvent::System => "System",
        };

        write!(f, "Async - {}", word)
    }
}

/// A simpler, less idiomatic and less understandable, but more direct,
/// representation of Gms2 event types and numbers. We use this internally in
/// the serde of the higher level `EventType` enum, which is also given.
///
/// This struct is made public largely so non-Rust applications downstream can
/// have an easier interface to work with. Rust applications are encouraged to
/// stick with the more idiomatic and user-friendly `EventType`, which is far
/// more type safe while being equally performant.
#[derive(
    Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Serialize, Deserialize, Copy, Clone,
)]
pub struct EventIntermediary {
    event_type: usize,
    event_num: usize,
}

impl From<&'_ EventType> for EventIntermediary {
    fn from(value: &'_ EventType) -> Self {
        Self::from(*value)
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
            EventType::CleanUp => EventIntermediary {
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
            EventType::Mouse(mouse_event) => EventIntermediary {
                event_type: 6,
                event_num: match mouse_event {
                    MouseEvent::Down(mb) => MouseEvent::DOWN_OFFSET + mb.event_offset(),
                    MouseEvent::Pressed(mb) => MouseEvent::PRESSED_OFFSET + mb.event_offset(),
                    MouseEvent::Released(mb) => MouseEvent::RELEASED_OFFSET + mb.event_offset(),
                    MouseEvent::NoInput => 3,
                    MouseEvent::MouseEnter => 10,
                    MouseEvent::MouseExit => 11,
                    MouseEvent::MouseWheelUp => 60,
                    MouseEvent::MouseWheelDown => 61,
                },
            },
            EventType::KeyDown(vk) => EventIntermediary {
                event_type: 5,
                event_num: vk as usize,
            },
            EventType::KeyPress(vk) => EventIntermediary {
                event_type: 9,
                event_num: vk as usize,
            },
            EventType::KeyRelease(vk) => EventIntermediary {
                event_type: 10,
                event_num: vk as usize,
            },
            EventType::Gesture(gv) => EventIntermediary {
                event_type: 13,
                event_num: gv.gesture as usize
                    + if gv.local {
                        0
                    } else {
                        GestureEvent::GLOBAL_OFFSET
                    },
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
            0 => {
                if o.event_num == 0 {
                    EventType::Create
                } else {
                    return Err(EventTypeConvertErrors::CannotFindEventNumber(0));
                }
            }
            1 => {
                if o.event_num == 0 {
                    EventType::Destroy
                } else {
                    return Err(EventTypeConvertErrors::CannotFindEventNumber(1));
                }
            }
            12 => {
                if o.event_num == 0 {
                    EventType::CleanUp
                } else {
                    return Err(EventTypeConvertErrors::CannotFindEventNumber(12));
                }
            }

            // step
            3 => match o.event_num {
                0 => EventType::Step(Stage::Main),
                1 => EventType::Step(Stage::Begin),
                2 => EventType::Step(Stage::End),
                _ => return Err(EventTypeConvertErrors::CannotFindEventNumber(3)),
            },

            2 => {
                if o.event_num <= EventType::ALARM_MAX {
                    EventType::Alarm(o.event_num)
                } else {
                    return Err(EventTypeConvertErrors::CannotFindEventNumber(2));
                }
            }

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

            4 => {
                if o.event_num == 0 {
                    EventType::Collision
                } else {
                    return Err(EventTypeConvertErrors::CannotFindEventNumber(4));
                }
            }

            6 => {
                if let Some(mouse_event) = MouseEvent::convert_to_input(o.event_num) {
                    EventType::Mouse(mouse_event)
                } else {
                    match o.event_num {
                        10 => EventType::Mouse(MouseEvent::MouseEnter),
                        11 => EventType::Mouse(MouseEvent::MouseExit),
                        60 => EventType::Mouse(MouseEvent::MouseWheelUp),
                        61 => EventType::Mouse(MouseEvent::MouseWheelDown),
                        _ => return Err(EventTypeConvertErrors::CannotFindEventNumber(6)),
                    }
                }
            }

            5 => {
                if let Some(vk) = FromPrimitive::from_usize(o.event_num) {
                    EventType::KeyDown(vk)
                } else {
                    return Err(EventTypeConvertErrors::CannotFindEventNumber(5));
                }
            }

            9 => {
                if let Some(vk) = FromPrimitive::from_usize(o.event_num) {
                    EventType::KeyPress(vk)
                } else {
                    return Err(EventTypeConvertErrors::CannotFindEventNumber(9));
                }
            }

            10 => {
                if let Some(vk) = FromPrimitive::from_usize(o.event_num) {
                    EventType::KeyRelease(vk)
                } else {
                    return Err(EventTypeConvertErrors::CannotFindEventNumber(10));
                }
            }

            13 => {
                if let Some(event) = GestureEvent::convert_to_input(o.event_num) {
                    EventType::Gesture(event)
                } else {
                    return Err(EventTypeConvertErrors::CannotFindEventNumber(13));
                }
            }

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
            _ => return Err(UnknownEventTypeNumber)?,
        };

        Ok(output)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[repr(usize)]
pub enum EventTypeNumber {
    Create = 0,
    Destroy = 1,
    Alarm = 2,
    Step = 3,
    Collision = 4,
    Keyboard = 5,
    Mouse = 6,
    Other = 7,
    Draw = 8,
    KeyPress = 9,
    KeyRelease = 10,
    CleanUp = 12,
    Gesture = 13,
}

impl std::str::FromStr for EventTypeNumber {
    type Err = UnknownEventTypeNumber;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let event_type = match s {
            "create" | "Create" => Self::Create,
            "destroy" | "Destroy" => Self::Destroy,
            "cleanUp" | "CleanUp" | "cleanup" | "Cleanup" => Self::CleanUp,
            "step" | "Step" => Self::Step,
            "alarm" | "Alarm" => Self::Alarm,
            "draw" | "Draw" => Self::Draw,
            "collision" | "Collision" => Self::Collision,
            "mouse" | "Mouse" => Self::Mouse,
            "keyboard" | "Keyboard" | "keyBoard" | "KeyBoard" | "keyDown" | "keydown" => {
                Self::Keyboard
            }
            "keypress" | "keyPress" | "KeyPress" | "Keypress" => Self::KeyPress,
            "keyrelease" | "keyRelease" | "KeyRelease" | "Keyrelease" => Self::KeyRelease,
            "gesture" | "Gesture" => Self::Gesture,
            "other" | "Other" => Self::Other,
            _ => return Err(UnknownEventTypeNumber),
        };

        Ok(event_type)
    }
}

impl fmt::Display for EventTypeNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = match self {
            EventTypeNumber::Create => "create",
            EventTypeNumber::Destroy => "destroy",
            EventTypeNumber::Alarm => "alarm",
            EventTypeNumber::Step => "step",
            EventTypeNumber::Collision => "collision",
            EventTypeNumber::Keyboard => "keyboard",
            EventTypeNumber::Mouse => "mouse",
            EventTypeNumber::Other => "other",
            EventTypeNumber::Draw => "draw",
            EventTypeNumber::KeyPress => "key_press",
            EventTypeNumber::KeyRelease => "key_release",
            EventTypeNumber::CleanUp => "clean_up",
            EventTypeNumber::Gesture => "gesture",
        };

        f.pad(word)
    }
}

impl From<&'_ EventType> for EventTypeNumber {
    fn from(value: &'_ EventType) -> Self {
        Self::from(*value)
    }
}

impl From<EventType> for EventTypeNumber {
    fn from(value: EventType) -> Self {
        match value {
            EventType::Create => EventTypeNumber::Create,
            EventType::Step(_) => EventTypeNumber::Step,
            EventType::Draw(_) => EventTypeNumber::Draw,
            EventType::Alarm(_) => EventTypeNumber::Alarm,
            EventType::Collision => EventTypeNumber::Collision,
            EventType::Mouse(_) => EventTypeNumber::Mouse,
            EventType::KeyDown(_) => EventTypeNumber::Keyboard,
            EventType::KeyPress(_) => EventTypeNumber::KeyPress,
            EventType::KeyRelease(_) => EventTypeNumber::KeyRelease,
            EventType::Gesture(_) => EventTypeNumber::Gesture,
            EventType::Other(_) => EventTypeNumber::Other,
            EventType::Async(_) => EventTypeNumber::Other,
            EventType::Destroy => EventTypeNumber::Destroy,
            EventType::CleanUp => EventTypeNumber::CleanUp,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, thiserror::Error)]
pub enum EventTypeConvertErrors {
    #[error("invalid event_number for event type `{0}`")]
    CannotFindEventNumber(usize),
    #[error(transparent)]
    CannotFindEventType(UnknownEventTypeNumber),
    #[error("string didn't follow pattern x_y")]
    BadString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("unknown event type number")]
pub struct UnknownEventTypeNumber;

impl From<UnknownEventTypeNumber> for EventTypeConvertErrors {
    fn from(v: UnknownEventTypeNumber) -> Self {
        Self::CannotFindEventType(v)
    }
}

mod event_type_serialization {
    use super::*;
    use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    enum Field {
        #[serde(rename = "eventNum")]
        Number,
        #[serde(rename = "eventType")]
        Type,
    }

    use serde::de::{Error, MapAccess, Visitor};
    struct DeserializerVisitor;

    impl<'de> Visitor<'de> for DeserializerVisitor {
        type Value = EventType;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(r#"a value of "eventNum""#)
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
                            return Err(Error::duplicate_field("eventNum"));
                        }
                        event_number = Some(map.next_value()?);
                    }
                    Field::Type => {
                        if event_type.is_some() {
                            return Err(Error::duplicate_field("eventType"));
                        }
                        event_type = Some(map.next_value()?);
                    }
                }
            }

            let event_intermediary = EventIntermediary {
                event_type: event_type.ok_or_else(|| Error::missing_field("eventType"))?,
                event_num: event_number.ok_or_else(|| Error::missing_field("eventNum"))?,
            };

            EventType::try_from(event_intermediary).map_err(Error::custom)
        }
    }

    impl Serialize for EventType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let val: EventIntermediary = (*self).into();
            let mut inter = serializer.serialize_struct("EventIntermediary", 2)?;
            inter.serialize_field("eventNum", &val.event_num)?;
            inter.serialize_field("eventType", &val.event_type)?;
            inter.end()
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
    const FIELD_NAMES: [&str; 2] = ["eventNum", "eventType"];
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
            r#"{"rev_padding":10,"eventNum":0,"eventType":0,"padding":10}"#
        )
    }

    #[test]
    fn basic_deserialize() {
        let wrapper_str =
            r#"{"rev_padding":10,"eventNum":0,"eventType":0,"ioop": "oop","padding":10}"#;
        let wrapper: Wrapper = serde_json::from_str(wrapper_str).unwrap();

        assert_eq!(
            wrapper,
            Wrapper {
                rev_padding: 10,
                event_type: EventType::Create,
                padding: 10
            }
        );

        let event_type_str = r#"{"eventNum":0,"eventType":0}"#;
        let event_type: EventType = serde_json::from_str(event_type_str).unwrap();

        assert_eq!(event_type, EventType::Create);

        let event_type_str = r#"{"eventNum":100,"eventType":1000}"#;
        let event_type: Result<EventType, _> = serde_json::from_str(event_type_str);
        assert!(event_type.is_err());
    }

    #[test]
    fn symmetry() {
        harness(EventType::Create);
        harness(EventType::Destroy);
        harness(EventType::CleanUp);

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

        for i in 0..MouseButtonCode::COUNT {
            let mouse_button_code = FromPrimitive::from_usize(i).unwrap();
            harness(EventType::Mouse(MouseEvent::Down(MouseButton {
                mb_code: mouse_button_code,
                local: true,
            })));
            harness(EventType::Mouse(MouseEvent::Pressed(MouseButton {
                mb_code: mouse_button_code,
                local: true,
            })));
            harness(EventType::Mouse(MouseEvent::Released(MouseButton {
                mb_code: mouse_button_code,
                local: true,
            })));

            let mouse_button_code = FromPrimitive::from_usize(i).unwrap();
            harness(EventType::Mouse(MouseEvent::Down(MouseButton {
                mb_code: mouse_button_code,
                local: false,
            })));
            harness(EventType::Mouse(MouseEvent::Pressed(MouseButton {
                mb_code: mouse_button_code,
                local: false,
            })));
            harness(EventType::Mouse(MouseEvent::Released(MouseButton {
                mb_code: mouse_button_code,
                local: false,
            })));
        }

        harness(EventType::Mouse(MouseEvent::NoInput));
        harness(EventType::Mouse(MouseEvent::MouseEnter));
        harness(EventType::Mouse(MouseEvent::MouseExit));

        for i in 0..200 {
            if let Some(vk) = num_traits::FromPrimitive::from_usize(i) {
                harness(EventType::KeyDown(vk));
                harness(EventType::KeyPress(vk));
                harness(EventType::KeyRelease(vk));
            }
        }

        for i in 0..Gesture::COUNT {
            let gesture = FromPrimitive::from_usize(i).unwrap();
            harness(EventType::Gesture(GestureEvent {
                gesture,
                local: true,
            }));
            harness(EventType::Gesture(GestureEvent {
                gesture,
                local: false,
            }));
        }

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
                    assert_eq!(
                        EventIntermediary::from(et),
                        ei,
                        "input: {{event_type:{}, event_num:{}}}",
                        event_type,
                        event_num
                    );
                }
            }
        }
    }

    #[test]
    fn filepath_symmetry() {
        let event_names = [
            "Create",
            "Destroy",
            "CleanUp",
            "Step",
            "Alarm",
            "Draw",
            "Collision",
            "Other",
            "Other",
            "Keyboard",
            "KeyPress",
            "KeyRelease",
            "Gesture",
        ];

        for name in event_names.iter() {
            for i in 0..200 {
                match EventType::parse_filename(name, i) {
                    Ok(event) => {
                        let fname = event.filename();
                        let (output_fname, event_number) = fname.rsplit_once('_').unwrap();
                        assert_eq!(output_fname, *name);
                        assert_eq!(event_number.parse::<usize>().unwrap(), i);
                    }
                    Err(e) => {
                        assert!(
                            matches!(e, EventTypeConvertErrors::CannotFindEventNumber(_)),
                            "input: {}, {} || got {}",
                            name,
                            i,
                            e
                        );
                    }
                }
            }
        }
    }
}
