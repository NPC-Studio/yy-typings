use super::ResourceType;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSprite {
    #[serde(rename = "GMSprite")]
    #[default]
    GmSprite,
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
    GmImageLayer,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteFrame {
    #[serde(rename = "GMSpriteFrame")]
    #[default]
    GmSpriteFrame,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmImage {
    #[serde(rename = "GMSpriteBitmap")]
    #[default]
    GmSpriteBitmap,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSequence {
    #[serde(rename = "GMSequence")]
    #[default]
    GmSequence,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteEvent {
    #[serde(rename = "KeyframeStore<MessageEventKeyframe>")]
    #[default]
    GmSpriteEvent,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteMoment {
    #[serde(rename = "KeyframeStore<MomentsEventKeyframe>")]
    #[default]
    GmSpriteEvent,
}
