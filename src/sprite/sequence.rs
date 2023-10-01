use super::{consts, FilesystemPath, ResourceVersion};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

create_guarded_uuid!(SpriteSequenceId);

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpriteSequence {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::Sequence, String, 1, 4>,

    /// Whether to autorecord the sequence. This will always be true for
    /// sprites.
    #[default = true]
    pub auto_record: bool,

    pub backdrop_height: u64,

    #[default(0.5)]
    pub backdrop_image_opacity: f32,
    pub backdrop_image_path: String,

    pub backdrop_width: u64,
    pub backdrop_x_offset: f32,
    pub backdrop_y_offset: f32,

    /// The sprite events, which are always the Default value of SpriteEvents,
    /// for the Sprite.
    pub events: SpriteEvents,

    #[default(None)]
    pub event_stub_script: Option<()>,

    /// This can be a `{}`, which basically is
    /// impossible for us to parse. So that's a mess.
    #[default(serde_json::Value::Object(serde_json::Map::new()))]
    pub event_to_function: serde_json::Value,

    /// The number of frames of the Sprite. GMS2 records this as an f32 due to
    /// its shared status between sequences -- this can be converted to a
    /// `usize` without issue.
    pub length: f32,

    /// Whether the origin of the sprite is locked in the GMS2
    /// Editor. It has no effect otherwise.
    pub lock_origin: bool,

    /// The sprite moments, which are always the Default value of SpriteMoments,
    /// for the Sprite.
    pub moments: SpriteMoments,

    /// Will always be One.
    #[default = 1]
    pub playback: usize,

    /// The playback speed of the Sequence in terms of the PlaybackSpeed type.
    pub playback_speed: f32,

    /// The type of the playback speed.
    pub playback_speed_type: PlaybackSpeed,

    #[default(true)]
    pub show_backdrop: bool,

    #[default(false)]
    pub show_backdrop_image: bool,

    /// The Units of time of the Sequence. It will always be 1 in a Sprite.
    #[default = 1]
    pub time_units: usize,

    /// The "tracks" which the Sprite has. There is only every One track for a
    /// Sprite.
    pub tracks: Vec<Track>,

    /// This denotes which range is visible. I am not clear what it means.
    pub visible_range: Option<VisibleRange>,

    /// The volume of the sequence. Always 1.
    #[default = 1.0]
    pub volume: f32,

    pub xorigin: i32,
    pub yorigin: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VisibleRange {
    pub x: f32,
    pub y: f32,
}

/// These are the "events" which a Sprite is subscribed to. It will always be
/// its default value. It exists due to sharing resources with Sequences.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct SpriteEvents {
    /// The name of the Resource Type. This is a C# generic, so this Serde
    /// typing may not be sufficent. Testing will have to be done.
    pub resource_type: consts::SpriteEvent,

    /// The resource version of the SpriteEvent.
    pub resource_version: ResourceVersion,

    /// The keyframes which a SpriteEvent is assigned to. Because Sprites do not
    /// have access to all the resources of a sequence, they are always
    /// subscribed to 0 keyframes.
    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<()>,
}

/// These are the "moments" which a Sprite is subscribed to. It will always be
/// its default value. This is due to sharing resources with Sequences.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct SpriteMoments {
    /// The name of the Resource Type. This is a C# generic, so this Serde
    /// typing may not be sufficent. Testing will have to be done.
    pub resource_type: consts::SpriteMoment,

    /// The resource version of the SpriteMoment. Currently, it is always "1.0".
    pub resource_version: ResourceVersion,

    /// The keyframes which a SpriteEvent is assigned to. Because Sprites do not
    /// have access to all the resources of a sequence, they are always
    /// subscribed to 0 keyframes.
    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<()>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::SpriteFramesTrack, consts::SpriteTrackName>,

    /// Appears to always be zero.
    pub builtin_name: usize,

    /// Always empty vec.
    pub events: Vec<()>,

    /// Appears to always be true.
    #[default = true]
    pub inherits_track_colour: bool,

    /// Appears to always be 1.
    #[default = 1]
    pub interpolation: usize,

    /// Always `false`.
    pub is_creation_track: bool,

    /// These are the keyframes of the animation.
    pub keyframes: SpriteKeyframes,

    /// Always empty vec.
    pub modifiers: Vec<()>,

    /// This field appears to always be null. For some reason.
    pub sprite_id: Option<()>,

    /// Appears to be always zero
    pub track_colour: usize,
    /// Always empty vec.
    pub tracks: Vec<()>,
    /// Appears to always be zero.
    pub traits: usize,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpriteKeyframes {
    pub resource_type: consts::SpriteKeyframes,
    pub resource_version: ResourceVersion,
    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<SpriteKeyframe>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ElementType {
    MessageEventKeyframe,
    MomentsEventKeyframe,
    SpriteFrameKeyframe,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct SpriteKeyframe {
    pub resource_type: consts::SpriteKeyframe,

    pub resource_version: ResourceVersion,

    #[serde(rename = "Channels")]
    pub channels: Channels,

    #[serde(rename = "Disabled")]
    pub disabled: bool,

    /// A SpriteSequenceId, apparently with no relation to any other ID.
    pub id: SpriteSequenceId,

    #[serde(rename = "IsCreationKey")]
    pub is_creation_key: bool,

    #[serde(rename = "Key")]
    pub key: f32,

    #[serde(rename = "Length")]
    #[default = 1.0]
    pub length: f32,

    #[serde(rename = "Stretch")]
    pub stretch: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Channels {
    #[serde(rename = "0")]
    pub zero: SpriteZeroChannel,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpriteZeroChannel {
    pub resource_type: consts::SpriteZeroChannel,
    pub resource_version: ResourceVersion,
    #[serde(rename = "Id")]
    pub id: FilesystemPath,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault, Copy, Clone, Eq)]
#[repr(u8)]
pub enum PlaybackSpeed {
    #[default]
    FramesPerSecond,
    FramesPerGameFrame,
}
