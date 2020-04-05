use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod resource {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
    pub enum SpriteBBoxMode {
        Automatic,
        FullImage,
        Manual,
    }

    #[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
    pub enum SpriteCollisionKind {
        Precise,
        Rectangle,
        Ellipse,
        Diamond,
        RotatedRectangle = 5,
    }

    #[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
    pub enum SpritePlaybackSpeed {
        FramesPerSecond,
        FramesPerGameFrame,
    }

    fn gm_sprite() -> String {
        "GMSprite".to_string()
    }

    fn sprite_type() -> usize {
        0
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ResourceSprite {
        /// Event GUID
        pub id: Uuid,

        /// ModelName. Always GMSprite.
        #[serde(default = "gm_sprite")]
        pub model_name: String,

        /// Version string. Currently 1.12.
        mvc: String,

        /// Resource Name
        name: String,

        // Collisions
        #[serde(rename = "bbox_bottom")]
        pub bbox_bottom: f64,
        #[serde(rename = "bbox_left")]
        pub bbox_left: f64,
        #[serde(rename = "bbox_right")]
        pub bbox_right: f64,
        #[serde(rename = "bbox_top")]
        pub bbox_top: f64,
        #[serde(rename = "bboxmode")]
        pub bboxmode: SpriteBBoxMode,
        #[serde(rename = "colkind")]
        pub colkind: SpriteCollisionKind,
        #[serde(rename = "coltolerance")]
        pub coltolerance: f64,
        pub edge_filtering: bool,

        #[serde(rename = "For3D")]
        pub for3d: bool,

        pub frames: Vec<Frame>,
        pub layers: Vec<ImageLayer>,

        pub grid_x: f64,
        pub grid_y: f64,
        pub height: f64,

        #[serde(rename = "HTile")]
        pub h_tile: bool,

        pub origin: f64,
        pub origin_locked: bool,
        pub playback_speed: f64,
        pub playback_speedtype: SpritePlaybackSpeed,
        pub premultiply_alpha: bool,

        pub sepmasks: bool,

        /// This is always null...I think.
        pub swatch_colours: Option<serde_json::Value>,

        pub swf_precision: f64,

        pub texture_group_id: Uuid,

        /// This is always 0. Why is it there? Who can know.
        #[serde(rename = "type", default = "sprite_type")]
        pub resource_sprite_type: usize,

        #[serde(rename = "VTile")]
        pub v_tile: bool,

        pub width: f64,
        #[serde(rename = "xorig")]
        pub xorig: f64,
        #[serde(rename = "yorig")]
        pub yorig: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Frame {
        /// The UUID of this Frame. Every SpriteImage within
        /// this Frame will have a FrameId which will be the same
        /// as this ID.
        pub id: Uuid,

        /// Always "GMSpriteFrame"
        pub model_name: String,

        /// Current always 1.0.
        pub mvc: String,

        /// This is the SpriteId for the ResourceSprite which owns this Frame.
        #[serde(rename = "SpriteId")]
        pub sprite_id: Uuid,

        /// This is the actual image you'll see in the game.
        /// It's a composite of the images below. It's LayerID will
        /// always be UUID::default, or 0000...0000, but it's
        /// FrameID will always == Self.Id.
        pub composite_image: SpriteImage,

        /// These are the images which compose the composite image.
        /// At the minimum, there will be one Image. It's LayerID will
        /// correspond to the LayerId of a Sprite above.
        pub images: Vec<SpriteImage>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SpriteImage {
        /// This ID seems to referenced nowhere else, and may not have any independent
        /// usage. It does not reference anything else, at the minimum.
        pub id: Uuid,

        /// The model name is always "GMSpriteImage"
        pub model_name: String,

        /// Currently 1.0
        pub mvc: String,

        /// This always corresponds to the FrameId which this SpriteImage is within.
        #[serde(rename = "FrameId")]
        pub frame_id: Uuid,

        /// This always corresponds to the LayerId which this SpriteImage corresponds to.
        #[serde(rename = "LayerId")]
        pub layer_id: Uuid,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ImageLayer {
        /// This UUID corresponds to the SpriteImage LayerId UUID.
        pub id: Uuid,

        /// Always "GMImageLayer"
        pub model_name: String,

        /// Currently "1.0"
        pub mvc: String,

        /// This is the SpriteId for the ResourceSprite which owns this ImageLayer.
        #[serde(rename = "SpriteId")]
        pub sprite_id: Uuid,

        pub blend_mode: usize,
        pub is_locked: bool,
        pub name: String,

        /// Between 0-100
        pub opacity: f64,
        pub visible: bool,
    }
}

/// GMS2 project file typings
#[derive(Debug, Serialize, Deserialize)]
pub struct Yyp {
    /// Unknown property, seems to always be an empty array
    pub configs: Vec<Option<serde_json::Value>>,
    /// Contains project GUID
    pub id: Uuid,
    /// Denotes whether this project uses drag and drop or not
    #[serde(rename = "IsDnDProject")]
    pub is_dn_d_project: bool,
    /// Always "GMProject"
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// A version number string, unknown use
    pub mvc: String,
    /// Allows for experimental JS editing. Unfinished or legacy feature. It's a secret.
    pub option_ecma: bool,
    /// Parent project, apparently non-public feature
    #[serde(rename = "parentProject")]
    pub parent_project: ParentProject,
    /// Contains all project resources (unordered)
    pub resources: Vec<YypResource>,

    /// An array of script GUID's
    pub script_order: Vec<String>,

    /// Unknown property, usually an empty string, unless you're making a tutorial
    /// in which case, shame upon your house
    pub tutorial: Option<String>,
}

/// Parent project entry of a YYP
///
/// Parent project, apparently non-public feature
#[derive(Debug, Serialize, Deserialize)]
pub struct ParentProject {
    /// Contains parent project resources
    #[serde(rename = "alteredResources")]
    pub altered_resources: Vec<YypResource>,
    /// Unkown property, usually an empty array
    #[serde(rename = "hiddenResources")]
    pub hidden_resources: Vec<YypResource>,
    /// GUID of the parent project
    pub id: Uuid,
    /// Describes object entry type.
    /// Always "GMParentProject"
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// A version number string, unknown use
    pub mvc: String,
    /// Contains parent project path presumably, always contains the following string:
    /// "${base_project}"
    #[serde(rename = "projectPath")]
    pub project_path: String,
}

/// Represents a resource entry in a YYP
#[derive(Debug, Serialize, Deserialize)]
pub struct YypResource {
    /// This resource entry GUID (not the GUID of the resource itself). Appears to serve no
    /// purpose.
    #[serde(rename = "Key")]
    pub key: Uuid,
    /// Contains resource information
    #[serde(rename = "Value")]
    pub value: YypResourceValue,
}

/// Contains resource information
#[derive(Debug, Serialize, Deserialize)]
pub struct YypResourceValue {
    /// Unknown property, seems to always be an empty array
    #[serde(rename = "configDeltaFiles")]
    pub config_delta_files: Vec<Option<serde_json::Value>>,
    /// Unknown property, seems to always be an empty array
    #[serde(rename = "configDeltas")]
    pub config_deltas: Vec<Option<serde_json::Value>>,
    /// GUID of the resource
    pub id: Uuid,
    /// Describes object entry type, which is always "GMResourceInfo" for YYPResources
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// A version number string, unknown use
    pub mvc: String,
    /// Unknown property, seems to always have only one entry: "default"
    #[serde(rename = "resourceCreationConfigs")]
    pub resource_creation_configs: Vec<String>,

    /// Contains the relative backslash-escaped path to the resource's .yy file
    #[serde(rename = "resourcePath")]
    pub resource_path: String,
    /// Describes the resource type
    #[serde(rename = "resourceType")]
    pub resource_type: ResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceType {
    #[serde(rename = "GMObject")]
    GmObject,
    #[serde(rename = "GMIncludedFile")]
    GmIncludedFile,
    #[serde(rename = "GMExtension")]
    GmExtension,
    #[serde(rename = "GMExtensionFile")]
    GmExtensionFile,
    #[serde(rename = "GMExtensionConstant")]
    GmExtensionConstant,
    #[serde(rename = "GMFont")]
    GmFont,
    #[serde(rename = "GMNote")]
    GmNote,
    #[serde(rename = "GMOption")]
    GmOption,
    #[serde(rename = "GMPath")]
    GmPath,
    #[serde(rename = "GMRoom")]
    GmRoom,
    #[serde(rename = "GMScript")]
    GmScript,
    #[serde(rename = "GMShader")]
    GmShader,
    #[serde(rename = "GMSound")]
    GmSound,
    #[serde(rename = "GMSprite")]
    GmSprite,
    #[serde(rename = "GMTileSet")]
    GmTileSet,
    #[serde(rename = "GMFolder")]
    GmFolder,
    #[serde(rename = "GMTimeline")]
    GmTimeline,
}

pub enum GmSprite {
    
}
