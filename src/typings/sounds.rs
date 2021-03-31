use std::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

use crate::AudioGroup;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sound {
    /// The type of compression for the file.
    pub compression: Compression,

    /// The volume of the file.
    pub volume: f64,

    /// Whether the sound is "preloaded" or not. I don't know what this
    /// actually does.
    pub preload: bool,
    pub bitrate: BitRate,
    pub sample_rate: u64,

    #[serde(rename = "type")]
    pub sound_type: usize,
    pub bit_depth: usize,

    /// This is the Path to the Audio Group Id.
    pub audio_group_id: AudioGroup,
    pub sound_file: String,
    duration: f64,

    /// The parent in the Gms2 virtual file system, ie. the parent which
    /// a user would see in the Navigation Pane in Gms2. This has no relationship
    /// to the actual operating system's filesystem.
    pub parent: crate::ViewPath,

    /// The resource version of this yy file. At default 1.0.
    pub resource_version: crate::ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: String,

    /// The tags given to the object.
    pub tags: crate::Tags,

    /// The resource type, always the same for sounds.
    pub resource_type: ConstGmSound,
}

#[derive(Debug, Copy, SmartDefault, Deserialize_repr, Serialize_repr, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum Compression {
    #[default]
    Uncompressed,
    Compressed,
    UncompressedOnLoad,
    CompressedStreamed,
}

#[derive(Debug, Copy, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[repr(transparent)]
#[serde(transparent)]
pub struct BitRate(u32);

impl BitRate {
    pub fn new(bitrate: u32) -> Result<Self, InvalidBitRate> {
        if Self::is_valid_bitrate(bitrate) {
            return Err(InvalidBitRate);
        }

        Ok(Self(bitrate))
    }

    pub fn is_valid_bitrate(bitrate: u32) -> bool {
        match bitrate {
            0..=64 => {
                if bitrate % 8 != 0 {
                    return false;
                }
            }
            80..=160 => {
                if bitrate % 16 != 0 {
                    return false;
                }
            }
            192..=256 => {
                if bitrate % 32 != 0 {
                    return false;
                }
            }
            320 | 512 => {}
            _ => {
                return false;
            }
        }

        true
    }
}

impl Default for BitRate {
    fn default() -> Self {
        Self(128)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidBitRate;

impl fmt::Display for InvalidBitRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("that was not a valid bit rate")
    }
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSound {
    #[serde(rename = "GMSound")]
    #[default]
    Const,
}
