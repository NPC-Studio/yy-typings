use super::ResourceType;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSprite {
    #[serde(rename = "GMSprite")]
    #[default]
    Const,
}

impl From<ConstGmSprite> for ResourceType {
    fn from(_: ConstGmSprite) -> Self {
        Self::GmSprite
    }
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmImageLayer {
    #[serde(rename = "GMImageLayer")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteFrame {
    #[serde(rename = "GMSpriteFrame")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmImage {
    #[serde(rename = "GMSpriteBitmap")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSequence {
    #[serde(rename = "GMSequence")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteEvent {
    #[serde(rename = "KeyframeStore<MessageEventKeyframe>")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteMoment {
    #[serde(rename = "KeyframeStore<MomentsEventKeyframe>")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteTrackName {
    #[serde(rename = "frames")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteFramesTrack {
    #[serde(rename = "GMSpriteFramesTrack")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteKeyframes {
    #[serde(rename = "KeyframeStore<SpriteFrameKeyframe>")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteKeyframe {
    #[serde(rename = "Keyframe<SpriteFrameKeyframe>")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteZeroChannel {
    #[serde(rename = "SpriteFrameKeyframe")]
    #[default]
    Const,
}
