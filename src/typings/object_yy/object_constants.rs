use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmObject {
    #[serde(rename = "GMObject")]
    #[default]
    Const,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmEvent {
    #[serde(rename = "GMEvent")]
    #[default]
    Const,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmObjectProperty {
    #[serde(rename = "GMObjectProperty")]
    #[default]
    Const,
}
