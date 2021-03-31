use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(
    Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone, Ord, PartialOrd,
)]
pub enum ConstGmObject {
    #[serde(rename = "GMObject")]
    #[default]
    Const,
}

#[derive(
    Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone, Ord, PartialOrd,
)]
pub enum ConstGmEvent {
    #[serde(rename = "GMEvent")]
    #[default]
    Const,
}

#[derive(
    Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone, Ord, PartialOrd,
)]
pub enum ConstGmObjectProperty {
    #[serde(rename = "GMObjectProperty")]
    #[default]
    Const,
}

#[derive(
    Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone, Ord, PartialOrd,
)]
pub enum ConstGmObjectOverrideProperty {
    #[serde(rename = "GMOverriddenProperty")]
    #[default]
    Const,
}
