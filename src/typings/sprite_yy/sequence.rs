use super::{sprite_constants::*, FilesystemPath, ResourceVersion, Tags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

create_guarded_uuid!(SpriteSequenceId);

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpriteSequence {
    /// The path to the parent sprite.
    pub sprite_id: FilesystemPath,

    /// The Units of time of the Sequence. It will always be 1 in a Sprite.
    #[default = 1]
    pub time_units: usize,

    /// Will always be One.
    #[default = 1]
    pub playback: usize,

    /// The playback speed of the Sequence in terms of the PlaybackSpeed type.
    pub playback_speed: f64,

    /// The type of the playback speed.
    pub playback_speed_type: PlaybackSpeed,

    /// Whether to autorecord the sequence. This will always be true for
    /// sprites.
    #[default = true]
    pub auto_record: bool,

    /// The volume of the sequence. Always 1.
    #[default = 1.0]
    pub volume: f64,

    /// The number of frames of the Sprite. GMS2 records this as an f64 due to
    /// its shared status between sequences -- this can be converted to a
    /// `usize` without issue.
    pub length: f64,

    /// The sprite events, which are always the Default value of SpriteEvents,
    /// for the Sprite.
    pub events: SpriteEvents,

    /// The sprite moments, which are always the Default value of SpriteMoments,
    /// for the Sprite.
    pub moments: SpriteMoments,

    /// The "tracks" which the Sprite has. There is only every One track for a
    /// Sprite.
    pub tracks: Vec<Track>,
    /// This denotes which range is visible. I am not clear what it means.
    pub visible_range: Option<VisibleRange>,
    /// Whether the origin of the sprite is locked in the GMS2
    /// Editor. It has no effect otherwise.
    pub lock_origin: bool,

    #[default(true)]
    pub show_backdrop: bool,
    #[default(false)]
    pub show_backdrop_image: bool,
    #[default(String::new())]
    pub backdrop_image_path: String,
    #[default(0.5)]
    pub backdrop_image_opacity: f64,
    pub backdrop_width: u64,
    pub backdrop_height: u64,
    pub backdrop_x_offset: f64,
    pub backdrop_y_offset: f64,
    pub xorigin: i32,
    pub yorigin: i32,

    /// This can be a `{}`, which basically is
    /// impossible for us to parse. So that's a mess.
    #[default(serde_json::Value::Object(serde_json::Map::new()))]
    pub event_to_function: serde_json::Value,
    #[default(None)]
    pub event_stub_script: Option<()>,
    /// This is a duplicate of `sprite_id`, and should always
    /// be the same value. It is unknown why there is duplicate data.
    pub parent: FilesystemPath,

    /// The resource version. Currently `1.3`.
    #[default("1.3".parse::<ResourceVersion>().unwrap())]
    pub resource_version: ResourceVersion,

    /// The name of the SpriteSequence, which is always an empty string.
    #[default(String::new())]
    pub name: String,

    /// The tags given to this resource. Empty.
    #[default(vec![])]
    pub tags: Tags,

    /// This is the resource type. Always GMSequence.
    pub resource_type: ConstGmSequence,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VisibleRange {
    pub x: f64,
    pub y: f64,
}

/// These are the "events" which a Sprite is subscribed to. It will always be
/// its default value. It exists due to sharing resources with Sequences.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpriteEvents {
    /// The keyframes which a SpriteEvent is assigned to. Because Sprites do not
    /// have access to all the resources of a sequence, they are always
    /// subscribed to 0 keyframes.
    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<()>,

    /// The resource version of the SpriteEvent.
    pub resource_version: ResourceVersion,

    /// The name of the Resource Type. This is a C# generic, so this Serde
    /// typing may not be sufficent. Testing will have to be done.
    pub resource_type: ConstGmSpriteEvent,
}

/// These are the "moments" which a Sprite is subscribed to. It will always be
/// its default value. This is due to sharing resources with Sequences.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpriteMoments {
    /// The keyframes which a SpriteEvent is assigned to. Because Sprites do not
    /// have access to all the resources of a sequence, they are always
    /// subscribed to 0 keyframes.
    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<()>,

    /// The resource version of the SpriteMoment. Currently, it is always "1.0".
    pub resource_version: ResourceVersion,

    /// The name of the Resource Type. This is a C# generic, so this Serde
    /// typing may not be sufficent. Testing will have to be done.
    pub resource_type: ConstGmSpriteMoment,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    /// The name of the track. The trackname is always "frames".
    pub name: ConstGmSpriteTrackName,

    /// This field appears to always be null. For some reason.
    pub sprite_id: Option<()>,

    /// These are the keyframes of the animation.
    pub keyframes: SpriteKeyframes,

    /// Appears to be always zero
    pub track_colour: usize,
    /// Appears to always be true.
    #[default = true]
    pub inherits_track_colour: bool,
    /// Appears to always be zero.
    pub builtin_name: usize,
    /// Appears to always be zero.
    pub traits: usize,
    /// Appears to always be 1.
    #[default = 1]
    pub interpolation: usize,
    /// Always empty vec.
    pub tracks: Vec<()>,
    /// Always empty vec.
    pub events: Vec<()>,
    /// Always empty vec.
    pub modifiers: Vec<()>,
    /// Always `false`.
    pub is_creation_track: bool,
    /// The resource version. Currently "1.0".
    pub resource_version: ResourceVersion,
    /// The tags, which cannot be assigned in IDE.
    pub tags: Tags,
    /// The resource type constant.
    pub resource_type: ConstGmSpriteFramesTrack,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpriteKeyframes {
    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<SpriteKeyframe>,
    pub resource_version: ResourceVersion,
    pub resource_type: ConstGmSpriteKeyframes,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct SpriteKeyframe {
    /// A SpriteSequenceId, apparently with no relation to any other ID.
    pub id: SpriteSequenceId,
    #[serde(rename = "Key")]
    pub key: f64,
    #[serde(rename = "Length")]
    #[default = 1.0]
    pub length: f64,
    #[serde(rename = "Stretch")]
    pub stretch: bool,
    #[serde(rename = "Disabled")]
    pub disabled: bool,
    #[serde(rename = "IsCreationKey")]
    pub is_creation_key: bool,
    #[serde(rename = "Channels")]
    pub channels: Channels,
    pub resource_version: ResourceVersion,
    pub resource_type: ConstGmSpriteKeyframe,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Channels {
    #[serde(rename = "0")]
    pub zero: SpriteZeroChannel,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpriteZeroChannel {
    #[serde(rename = "Id")]
    pub id: FilesystemPath,
    pub resource_version: ResourceVersion,
    pub resource_type: ConstGmSpriteZeroChannel,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault, Copy, Clone, Eq)]
#[repr(u8)]
pub enum PlaybackSpeed {
    #[default]
    FramesPerSecond,
    FramesPerGameFrame,
}
