use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

use crate::{AudioGroupId, ResourceData};

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

    /// The bitrate of the audio. Higher is better I think? Honestly lol,
    /// knowing what "bitrate" means is for fuckin nerds
    /// This is in "kbps" apparently, so probably kilobits (bytes?) per second.
    /// Look, no one knows.
    pub bit_rate: BitRate,

    /// SAMPLE RATE?? I didn't know BITRATE you think i'm gonna know "SAMPLE RATE"
    /// it's the rate of the samples go fuck yourself
    pub sample_rate: SampleRate,

    /// The kind of the sound for mono/stereo.
    #[serde(rename = "type")]
    pub output: Output,

    /// The quality of the sound.
    pub bit_depth: BitDepth,

    /// This is the Path to the Audio Group Id.
    pub audio_group_id: AudioGroupId,

    /// This is a path to the Audio file, which will be the same name as the sound file generally.
    /// If there is no sound set up for this asset, then this field **will be an empty string.**
    pub sound_file: String,

    /// The duration of the sound in seconds, such as `12.4` for 12 seconds and 400 miliseconds.
    pub duration: f64,

    /// Common resource data.
    #[serde(flatten)]
    pub resource_data: ResourceData,

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

#[derive(Debug, Copy, SmartDefault, Deserialize_repr, Serialize_repr, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum BitDepth {
    #[default]
    EightBit,
    SixteenBit,
}

#[derive(Debug, Copy, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[repr(transparent)]
#[serde(transparent)]
pub struct BitRate(u32);

impl BitRate {
    pub fn new(bitrate: u32) -> Option<Self> {
        if Self::is_valid_bitrate(bitrate) {
            return None;
        }

        Some(Self(bitrate))
    }

    pub fn set_bitrate(&mut self, rate: u32) -> Result<(), InvalidBitRate> {
        if Self::is_valid_bitrate(rate) {
            self.0 = rate;
            Ok(())
        } else {
            Err(InvalidBitRate)
        }
    }

    pub fn bitrate(&self) -> u32 {
        self.0
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

#[derive(Debug, Clone, Copy)]
pub struct InvalidBitRate;

impl std::fmt::Display for InvalidBitRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Bit Rate")
    }
}
impl std::error::Error for InvalidBitRate {}

#[derive(Debug, Copy, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[repr(transparent)]
#[serde(transparent)]
pub struct SampleRate(u32);

impl SampleRate {
    pub fn new(bitrate: u32) -> Option<Self> {
        if Self::is_valid_sample_rate(bitrate) {
            return None;
        }

        Some(Self(bitrate))
    }

    pub fn is_valid_sample_rate(bitrate: u32) -> bool {
        matches!(bitrate, 5512 | 11025 | 22050 | 32000 | 44100 | 48000)
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) -> Result<(), InvalidSampleRate> {
        if Self::is_valid_sample_rate(sample_rate) {
            self.0 = sample_rate;
            Ok(())
        } else {
            Err(InvalidSampleRate)
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.0
    }

    pub fn valid_sample_rates() -> [u32; 6] {
        [5512, 11025, 22050, 32000, 44100, 48000]
    }
}

impl Default for SampleRate {
    fn default() -> Self {
        Self(128)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InvalidSampleRate;

impl std::fmt::Display for InvalidSampleRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Sample Rate")
    }
}
impl std::error::Error for InvalidSampleRate {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, SmartDefault)]
#[repr(u8)]
pub enum Output {
    #[default]
    Mono,
    Stereo,
    ThreeDee,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSound {
    #[serde(rename = "GMSound")]
    #[default]
    Const,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        let input = r#"
        {
            "compression": 2,
            "volume": 1.0,
            "preload": false,
            "bitRate": 128,
            "sampleRate": 44100,
            "type": 0,
            "bitDepth": 1,
            "audioGroupId": {
              "name": "audiogroup_default",
              "path": "audiogroups/audiogroup_default"
            },
            "soundFile": "snd_summer_ambiance_day.mp3",
            "duration": 60.0583344,
            "parent": {
              "name": "Sounds",
              "path": "folders/Sounds.yy"
            },
            "resourceVersion": "1.0",
            "name": "snd_summer_ambiance_day",
            "tags": [],
            "resourceType": "GMSound"
          }"#;

        let _: Sound = serde_json::from_str(input).unwrap();
    }
}
