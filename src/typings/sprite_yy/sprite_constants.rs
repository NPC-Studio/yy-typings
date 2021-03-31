use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSprite {
    #[serde(rename = "GMSprite")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmImageLayer {
    #[serde(rename = "GMImageLayer")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSpriteFrame {
    #[serde(rename = "GMSpriteFrame")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmImage {
    #[serde(rename = "GMSpriteBitmap")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSequence {
    #[serde(rename = "GMSequence")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSpriteEvent {
    #[serde(rename = "KeyframeStore<MessageEventKeyframe>")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSpriteMoment {
    #[serde(rename = "KeyframeStore<MomentsEventKeyframe>")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSpriteTrackName {
    #[serde(rename = "frames")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSpriteFramesTrack {
    #[serde(rename = "GMSpriteFramesTrack")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSpriteKeyframes {
    #[serde(rename = "KeyframeStore<SpriteFrameKeyframe>")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSpriteKeyframe {
    #[serde(rename = "Keyframe<SpriteFrameKeyframe>")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmSpriteZeroChannel {
    #[serde(rename = "SpriteFrameKeyframe")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmNineSliceData {
    #[serde(rename = "GMNineSliceData")]
    #[default]
    Const,
}
