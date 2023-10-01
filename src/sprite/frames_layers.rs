use super::{consts, FilesystemPath, ResourceVersion};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

create_guarded_uuid!(FrameId);
create_guarded_uuid!(LayerId);

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// Although named FrameId, this is actually the path to the the parent
    /// frame resource. The `name` field will correspond to the `Frame.name`
    /// field.
    #[serde(rename = "FrameId")]
    pub frame_id: FilesystemPath,

    /// This always corresponds to the LayerId which this SpriteImage
    /// corresponds to. It will be `None` in the case of the
    /// `composite_image` field -- otherwise, it will contain a valid path
    /// to the parent layer.
    #[serde(rename = "LayerId")]
    pub layer_id: Option<FilesystemPath>,

    /// The version of the resource.
    pub resource_version: ResourceVersion,

    /// This appears to only ever be two values:
    ///
    /// - `None` for normal images
    /// - `Some("composite")` for the composite image.
    ///
    /// It may have other purposes.
    pub name: Option<String>,

    /// The tags assigned to each image. You can apparently tag an image.
    pub tags: Vec<String>,

    /// The resource name of the GM Image.
    pub resource_type: consts::Image,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpriteLayer {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::ImageLayer, LayerId>,

    /// Defines the blendmode in the GMS2 editor. @todo Must be typed at a later
    /// date.
    pub blend_mode: usize,

    /// This is the actual name shown in the GMS2 Sprite Editor.
    #[default("default".to_string())]
    pub display_name: String,

    /// Defines if the layer is locked to editing. Only has an effect on the
    /// GMS2 Sprite Editor.
    pub is_locked: bool,

    /// Between 0-100
    #[default = 100.0]
    pub opacity: f64,

    /// Defines the visibility of the layer. It will default to true on
    /// import. It is changed in the GMS2 Sprite Editor.
    pub visible: bool,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Debug,
    SmartDefault,
    Copy,
    Clone,
    Eq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum BlendMode {
    #[default]
    Normal,
    Add,
    Subtract,
    Multiply,
}
