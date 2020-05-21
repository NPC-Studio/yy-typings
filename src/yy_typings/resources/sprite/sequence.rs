use super::{sprite_constants::*, ParentPath, Tags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpriteSequence {
    /// The path to the parent sprite.
    pub sprite_id: ParentPath,

    /// The Units of time of the Sequence. It will always be 1 in a Sprite.
    pub time_units: usize,

    /// Will always be One.
    pub playback: usize,

    /// The playback speed of the Sequence.
    pub playback_speed: f64,

    /// The type of the playback speed.
    pub playback_speed_type: PlaybackSpeed,

    /// Whether to autorecord the sequence. This will always be true for sprites.
    pub autorecord: bool,

    /// The volume of the sequence. Always 1.
    pub volume: f64,

    /// The number of frames of the Sprite. GMS2 records this as an f64 due to its shared
    /// status between sequences -- this can be converted to a `usize` without issue.
    pub length: f64,

    /// The sprite events, which are always the Default value of SpriteEvents, for the Sprite.
    pub events: SpriteEvents,

    /// The sprite moments, which are always the Default value of SpriteMoments, for the Sprite.
    pub moments: SpriteMoments,

    /// The "tracks" which the Sprite has. Each track corresponds to one of the frames of the animation.
    pub tracks: Vec<()>,
    pub visible_range: Option<()>,
    /// Whether the origin of the sprite is locked in the GMS2
    /// Editor. It has no effect otherwise.
    pub lock_origin: bool,
    pub show_backdrop: bool,
    pub show_backdrop_image: bool,
    pub backdrop_image_ath: (),
    pub backdrop_image_opacity: f64,
    pub backdrop_width: f64,
    pub backdrop_height: f64,
    pub backdrop_x_offset: f64,
    pub backdrop_y_offset: f64,
    pub xorig: isize,
    pub yorig: isize,

    /// This can be a `{}`, which basically is
    /// impossible for us to parse. So that's a mess.
    pub event_to_function: serde_json::Value,

    pub event_stub_script: Option<()>,
    /// This is a duplicate of `sprite_id`, and should always
    /// be the same value. It is unknown why there is duplicate data.
    pub parent: ParentPath,

    /// The resource version. Currently `1.3`.
    pub resource_version: String,

    /// The name of the sequence. It *can* be an empty string,
    /// but does not appear to ever be `null`.
    pub name: String,

    /// The tags given to this resource.
    pub tags: Tags,

    /// This is the resource type. Always GMSequence.
    pub resource_type: ConstGmSequence,
}

/// These are the "events" which a Sprite is subscribed to. It will always be
/// a specific default:
///
/// ```
/// SpriteEvents {
///     key_frames: vec![],
///     resource_version: "1.0",
///     resource_type: "KeyframeStore<MessageEventKeyframe>",
/// }
///```
/// This is due to sharing resources with Sequences.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpriteEvents {
    /// The keyframes which a SpriteEvent is assigned to. Because Sprites do not have access to all the resources
    /// of a sequence, they are always subscribed to 0 keyframes.
    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<()>,

    /// The resource version of the SpriteEvent. Currently, it is always "1.0".
    pub resource_version: String,

    /// The name of the Resource Type. This is a C# generic, so this Serde typing may not
    /// be sufficent. Testing will have to be done.
    pub resource_type: ConstGmSpriteEvent,
}
/// These are the "moments" which a Sprite is subscribed to. It will always be
/// its default value. This is due to sharing resources with Sequences.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpriteMoments {
    /// The keyframes which a SpriteEvent is assigned to. Because Sprites do not have access to all the resources
    /// of a sequence, they are always subscribed to 0 keyframes.
    #[serde(rename = "Keyframes")]
    pub keyframes: Vec<()>,

    /// The resource version of the SpriteMoment. Currently, it is always "1.0".
    pub resource_version: String,

    /// The name of the Resource Type. This is a C# generic, so this Serde typing may not
    /// be sufficent. Testing will have to be done.
    pub resource_type: ConstGmSpriteMoment,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Track {}

/*
"tracks": [
    {
    "name": "frames",
    "spriteId": null,
    "keyframes": {
        "Keyframes": [
        {
            "id": "7ac201d7-c56c-400f-8f97-c42ad0d98fba",
            "Key": 0.0,
            "Length": 1.0,
            "Stretch": false,
            "Disabled": false,
            "IsCreationKey": false,
            "Channels": {
            "0": {
                "Id": {
                "name": "7669a695-40b5-47eb-a089-f81c0f6be6b8",
                "path": "sprites/spr_jack/spr_jack.yy",
                },
                "resourceVersion": "1.0",
                "resourceType": "SpriteFrameKeyframe",
            },
            },
            "resourceVersion": "1.0",
            "resourceType": "Keyframe<SpriteFrameKeyframe>",
        },
        {
            "id": "6eb8b467-8d0d-46c4-8c69-5ed38acb63d5",
            "Key": 1.0,
            "Length": 1.0,
            "Stretch": false,
            "Disabled": false,
            "IsCreationKey": false,
            "Channels": {
            "0": {
                "Id": {
                "name": "dea3de44-c1aa-40ed-9cf6-0239bbb93ed4",
                "path": "sprites/spr_jack/spr_jack.yy",
                },
                "resourceVersion": "1.0",
                "resourceType": "SpriteFrameKeyframe",
            },
            },
            "resourceVersion": "1.0",
            "resourceType": "Keyframe<SpriteFrameKeyframe>",
        },
        {
            "id": "16876ed7-cd5c-4b54-b03a-0ff2d71dc308",
            "Key": 2.0,
            "Length": 1.0,
            "Stretch": false,
            "Disabled": false,
            "IsCreationKey": false,
            "Channels": {
            "0": {
                "Id": {
                "name": "c384f179-901a-4abb-b7a5-cae602f8150b",
                "path": "sprites/spr_jack/spr_jack.yy",
                },
                "resourceVersion": "1.0",
                "resourceType": "SpriteFrameKeyframe",
            },
            },
            "resourceVersion": "1.0",
            "resourceType": "Keyframe<SpriteFrameKeyframe>",
        },
        {
            "id": "d457b008-1cb9-43a5-8024-7576b09a0421",
            "Key": 3.0,
            "Length": 1.0,
            "Stretch": false,
            "Disabled": false,
            "IsCreationKey": false,
            "Channels": {
            "0": {
                "Id": {
                "name": "a52625ab-2499-4b46-9785-58ee50a1b048",
                "path": "sprites/spr_jack/spr_jack.yy",
                },
                "resourceVersion": "1.0",
                "resourceType": "SpriteFrameKeyframe",
            },
            },
            "resourceVersion": "1.0",
            "resourceType": "Keyframe<SpriteFrameKeyframe>",
        },
        ],
        "resourceVersion": "1.0",
        "resourceType": "KeyframeStore<SpriteFrameKeyframe>",
    },
    "trackColour": 0,
    "inheritsTrackColour": true,
    "builtinName": 0,
    "traits": 0,
    "interpolation": 1,
    "tracks": [],
    "events": [],
    "modifiers": [],
    "isCreationTrack": false,
    "resourceVersion": "1.0",
    "tags": [],
    "resourceType": "GMSpriteFramesTrack",
    },
],
*/

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault, Copy, Clone)]
#[repr(u8)]
pub enum PlaybackSpeed {
    #[default]
    FramesPerSecond,
    FramesPerGameFrame,
}
