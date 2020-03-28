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

    /// Bare bones interface for now. A Sprite has a lot more
    /// to it than just the BaseResource. Will be added soon.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResourceSprite {
        /// Event GUID
        pub id: Uuid,

        /// ModelName. Always GMSprite.
        #[serde(rename = "modelName", default = "gm_sprite")]
        pub model_name: String,

        /// Version string
        mvc: String,

        /// Resource Name
        name: String,

        // Collisions
        pub bbox_bottom: f64,
        pub bbox_left: f64,
        pub bbox_right: f64,
        pub bbox_top: f64,
        pub bboxmode: SpriteBBoxMode,
        pub colkind: SpriteCollisionKind,
        pub coltolerance: f64,

        #[serde(rename = "edgeFiltering")]
        pub edge_filtering: bool,

        #[serde(rename = "For3D")]
        pub for3d: bool,

        pub frames: Vec<Frame>,
        pub layers: Vec<ImageLayer>,

        #[serde(rename = "gridX")]
        pub grid_x: f64,
        #[serde(rename = "gridY")]
        pub grid_y: f64,
        pub height: f64,

        #[serde(rename = "HTile")]
        pub h_tile: bool,

        pub origin: f64,
        #[serde(rename = "originLocked")]
        pub origin_locked: bool,
        #[serde(rename = "playbackSpeed")]
        pub playback_speed: f64,
        #[serde(rename = "playbackSpeedtype")]
        pub playback_speedtype: SpritePlaybackSpeed,
        #[serde(rename = "premultiplyAlpha")]
        pub premultiply_alpha: bool,

        pub sepmasks: bool,

        /// This is always null...I think.
        #[serde(rename = "swatchColours")]
        pub swatch_colours: Option<serde_json::Value>,

        #[serde(rename = "swfPrecision")]
        pub swf_precision: f64,

        #[serde(rename = "textureGroupId")]
        pub texture_group_id: Uuid,

        /// This is always 0. Why is it there? Who can know.
        #[serde(rename = "type", default = "sprite_type")]
        pub resource_sprite_type: usize,

        #[serde(rename = "VTile")]
        pub v_tile: bool,

        pub width: f64,
        pub xorig: f64,
        pub yorig: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Frame {
        pub id: Uuid,
        pub mvc: String,

        /// This is the actual image you'll see in the game.
        /// It's a composite of the images below. It's UUID will
        /// always be UUID::default.
        pub composite_image: SpriteImage,

        /// These are the images which compose the composite image.
        ///
        pub images: Vec<SpriteImage>,

        /// Always "GMSpriteFrame"
        pub model_name: String,
        #[serde(rename = "SpriteId")]
        pub sprite_id: Uuid,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SpriteImage {
        pub id: Uuid,
        pub model_name: String,
        pub mvc: String,
        #[serde(rename = "FrameId")]
        pub frame_id: Uuid,
        #[serde(rename = "LayerId")]
        pub layer_id: Uuid,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ImageLayer {
        pub id: Uuid,
        pub model_name: String,
        pub mvc: String,
        #[serde(rename = "SpriteId")]
        pub sprite_id: Uuid,
        pub blend_mode: usize,
        pub is_locked: bool,
        pub name: String,
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
    pub id: String,
    /// Denotes whether this project uses drag and drop or not
    #[serde(rename = "IsDnDProject")]
    pub is_dn_d_project: bool,
    /// Usually contains resource type, in this case GMProject
    #[serde(rename = "modelName")]
    pub model_name: YypModelName,
    /// A version number string, unknown use
    pub mvc: String,
    /// Allows for experimental JS editing. Unfinished or legacy feature. It's a secret.
    pub option_ecma: bool,
    /// Parent project, apparently non-public feature
    #[serde(rename = "parentProject")]
    pub parent_project: ParentProject,
    /// Contains all project resources (unordered)
    pub resources: Vec<YypResource>,
    /// An array of script GUID's, seemingly optional
    pub script_order: Option<Vec<String>>,
    /// Unknown property, usually an empty string
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
    pub id: String,
    /// Describes object entry type.
    #[serde(rename = "modelName")]
    pub model_name: ParentProjectModelName,
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
    pub key: String,
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
    pub id: String,
    /// Describes object entry type, which is always "GMResourceInfo" for YYPResources
    #[serde(rename = "modelName")]
    pub model_name: FluffyModelName,
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

/// Internal resource type descriptor (GMEvent)
///
/// Describes the resource type
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceType {
    #[serde(rename = "GMExtension")]
    GmExtension,
    #[serde(rename = "GMExtensionConstant")]
    GmExtensionConstant,
    #[serde(rename = "GMExtensionFile")]
    GmExtensionFile,
    #[serde(rename = "GMExtensionFunction")]
    GmExtensionFunction,
    #[serde(rename = "GMFolder")]
    GmFolder,
    #[serde(rename = "GMFont")]
    GmFont,
    #[serde(rename = "GMIncludedFile")]
    GmIncludedFile,
    #[serde(rename = "GMNote")]
    GmNote,
    #[serde(rename = "GMObject")]
    GmObject,
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
    #[serde(rename = "GMTimeline")]
    GmTimeline,
}
