use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ShaderLanguageType = f64;
pub type PseudoBoolean = f64;
pub type SoundKind = f64;
pub type SoundType = f64;
pub type ExtensionFunctionType = f64;
pub type PathKind = f64;
pub type SpriteBBoxMode = f64;
pub type SpriteCollisionKind = f64;
pub type SpritePlaybackSpeed = f64;
pub type EventType = f64;
pub type EventNumber = f64;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceBaseResource {
    /// Event GUID
    pub id: String,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceModelNames,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceObject {
    /// List of object events
    #[serde(rename = "eventList")]
    pub event_list: Vec<ResourceObjectEvent>,
    /// Event GUID
    pub id: String,
    /// GUID of sprite mask
    #[serde(rename = "maskSpriteId")]
    pub mask_sprite_id: String,
    /// Internal resource type descriptor (GMObject)
    #[serde(rename = "modelName")]
    pub model_name: ResourceObjectModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    /// Properties overriden.
    #[serde(rename = "overriddenProperties")]
    pub overridden_properties: Option<Vec<HashMap<String, Option<serde_json::Value>>>>,
    /// GUID of parent object
    #[serde(rename = "parentObjectId")]
    pub parent_object_id: String,
    /// Indicates if this object is persistent
    pub persistent: bool,
    #[serde(rename = "physicsAngularDamping")]
    pub physics_angular_damping: f64,
    #[serde(rename = "physicsDensity")]
    pub physics_density: f64,
    #[serde(rename = "physicsFriction")]
    pub physics_friction: f64,
    #[serde(rename = "physicsGroup")]
    pub physics_group: f64,
    #[serde(rename = "physicsKinematic")]
    pub physics_kinematic: bool,
    #[serde(rename = "physicsLinearDamping")]
    pub physics_linear_damping: f64,
    /// Indicates if this object uses physics
    #[serde(rename = "physicsObject")]
    pub physics_object: bool,
    #[serde(rename = "physicsRestitution")]
    pub physics_restitution: f64,
    #[serde(rename = "physicsSensor")]
    pub physics_sensor: bool,
    #[serde(rename = "physicsShape")]
    pub physics_shape: f64,
    #[serde(rename = "physicsShapePoints")]
    pub physics_shape_points: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "physicsStartAwake")]
    pub physics_start_awake: bool,
    /// Properties are variables set using the "variables" box in the IDE.
    pub properties: Option<Vec<HashMap<String, Option<serde_json::Value>>>>,
    /// Indicates if this object is solid
    pub solid: bool,
    /// GUID of this object's sprite
    #[serde(rename = "spriteId")]
    pub sprite_id: String,
    /// Indicates if this object is visible
    pub visible: bool,
}

/// Describes the .gml file for each moment. Coded as a Create event.
/// Yes, this is how it is spelled.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceObjectEvent {
    /// Presumably, this holds the GUID of the other object if this were a collision event
    #[serde(rename = "collisionObjectId")]
    pub collision_object_id: String,
    /// Internal sub-event number
    pub enumb: f64,
    /// Internal Event number
    pub eventtype: f64,
    /// Event GUID
    pub id: String,
    /// Indicates if this event is drag and drop
    #[serde(rename = "IsDnD")]
    pub is_dn_d: bool,
    /// GUID of the object that owns this event (Can belong to parent object)
    pub m_owner: String,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// Version string, unknown use
    pub mvc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceOptions {
    /// Event GUID
    pub id: String,
    /// Internal resource type descriptor
    #[serde(rename = "modelName")]
    pub model_name: ResourceOptionsModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceRoom {
    /// The name of the Room Creation code FP, relative to the Room folder itself.
    #[serde(rename = "creationCodeFile")]
    pub creation_code_file: String,
    /// Event GUID
    pub id: String,
    /// Inherit Code from a Parent Room
    #[serde(rename = "inheritCode")]
    pub inherit_code: bool,
    /// Inherit Creation Order from a Parent Room
    #[serde(rename = "inheritCreationOrder")]
    pub inherit_creation_order: bool,
    /// Inherit Layers from a Parent Room
    #[serde(rename = "inheritLayers")]
    pub inherit_layers: bool,
    /// instanceCreationOrderIDs
    #[serde(rename = "instanceCreationOrderIDs")]
    pub instance_creation_order_i_ds: Vec<String>,
    #[serde(rename = "isDnD")]
    pub is_dn_d: bool,
    /// All your layers are placed here.
    pub layers: Vec<ResourceLayer>,
    /// Internal resource type descriptor
    #[serde(rename = "modelName")]
    pub model_name: ResourceRoomModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    /// Parent Room ID
    #[serde(rename = "parentID")]
    pub parent_id: String,
    /// Physics setting of the room.
    #[serde(rename = "physicsSettings")]
    pub physics_settings: ResourcePhysicsSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceLayer {
    #[serde(rename = "__type")]
    pub resource_layer_type: String,
    pub depth: f64,
    pub grid_x: f64,
    pub grid_y: f64,
    #[serde(rename = "hierarchyFrozen")]
    pub hierarchy_frozen: bool,
    #[serde(rename = "hierarchyVisible")]
    pub hierarchy_visible: bool,
    /// Event GUID
    pub id: String,
    #[serde(rename = "inheritLayerDepth")]
    pub inherit_layer_depth: bool,
    #[serde(rename = "inheritLayerSettings")]
    pub inherit_layer_settings: bool,
    #[serde(rename = "inheritSubLayers")]
    pub inherit_sub_layers: bool,
    #[serde(rename = "inheritVisibility")]
    pub inherit_visibility: bool,
    pub instances: Vec<ResourceInstance>,
    pub layers: Vec<ResourceLayer>,
    #[serde(rename = "m_parentID")]
    pub m_parent_id: String,
    #[serde(rename = "m_serialiseFrozen")]
    pub m_serialise_frozen: bool,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceModelNames,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    pub userdefined_depth: bool,
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceInstance {
    pub colour: ResourceColour,
    #[serde(rename = "creationCodeFile")]
    pub creation_code_file: String,
    #[serde(rename = "creationCodeType")]
    pub creation_code_type: String,
    /// Event GUID
    pub id: String,
    pub ignore: bool,
    #[serde(rename = "inheritCode")]
    pub inherit_code: bool,
    #[serde(rename = "inheritItemSettings")]
    pub inherit_item_settings: bool,
    #[serde(rename = "IsDnD")]
    pub is_dn_d: bool,
    #[serde(rename = "m_originalParentID")]
    pub m_original_parent_id: String,
    #[serde(rename = "m_serialiseFrozen")]
    pub m_serialise_frozen: bool,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceModelNames,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    pub name_with_no_file_rename: String,
    #[serde(rename = "objId")]
    pub obj_id: String,
    pub properties: Option<serde_json::Value>,
    pub rotation: f64,
    #[serde(rename = "scaleX")]
    pub scale_x: f64,
    #[serde(rename = "scaleY")]
    pub scale_y: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceColour {
    #[serde(rename = "Value")]
    pub value: f64,
}

/// Physics setting of the room.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcePhysicsSettings {
    pub id: String,
    #[serde(rename = "inheritPhysicsSettings")]
    pub inherit_physics_settings: bool,
    #[serde(rename = "modelName")]
    pub model_name: String,
    pub mvc: String,
    #[serde(rename = "PhysicsWorld")]
    pub physics_world: bool,
    #[serde(rename = "PhysicsWorldGravityX")]
    pub physics_world_gravity_x: f64,
    #[serde(rename = "PhysicsWorldGravityY")]
    pub physics_world_gravity_y: f64,
    #[serde(rename = "PhysicsWorldPixToMeters")]
    pub physics_world_pix_to_meters: f64,
}

/// We type it as "roomSetting" to reflect the .YY's typing of it.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceRoomSettings {
    #[serde(rename = "Height")]
    pub height: f64,
    pub id: String,
    #[serde(rename = "inheritRoomSettings")]
    pub inherit_room_settings: bool,
    #[serde(rename = "modelName")]
    pub model_name: String,
    pub mvc: String,
    pub persistent: bool,
    #[serde(rename = "Width")]
    pub width: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceView {
    pub hborder: f64,
    pub hport: f64,
    pub hspeed: f64,
    pub hview: f64,
    pub id: String,
    pub inherit: bool,
    #[serde(rename = "modelName")]
    pub model_name: String,
    pub mvc: String,
    #[serde(rename = "objId")]
    pub obj_id: String,
    pub vborder: f64,
    pub visible: bool,
    pub vspeed: f64,
    pub wport: f64,
    pub wview: f64,
    pub xport: f64,
    pub xview: f64,
    pub yport: f64,
    pub yview: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceViewSettings {
    #[serde(rename = "clearDisplayBuffer")]
    pub clear_display_buffer: bool,
    #[serde(rename = "clearViewBackground")]
    pub clear_view_background: bool,
    #[serde(rename = "enableViews")]
    pub enable_views: bool,
    pub id: String,
    #[serde(rename = "inheritViewSettings")]
    pub inherit_view_settings: bool,
    #[serde(rename = "modelName")]
    pub model_name: String,
    pub mvc: String,
}

/// Bare bones interface for now. A Sprite has a lot more
/// to it than just the BaseResource. Will be added soon.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceSprite {
    pub bbox_bottom: f64,
    pub bbox_left: f64,
    pub bbox_right: f64,
    pub bbox_top: f64,
    pub bboxmode: f64,
    pub colkind: f64,
    pub coltolerance: f64,
    #[serde(rename = "edgeFiltering")]
    pub edge_filtering: bool,
    #[serde(rename = "For3D")]
    pub for3_d: bool,
    pub frames: Vec<ResourceFrame>,
    #[serde(rename = "gridX")]
    pub grid_x: f64,
    #[serde(rename = "gridY")]
    pub grid_y: f64,
    pub height: f64,
    #[serde(rename = "HTile")]
    pub h_tile: bool,
    /// Event GUID
    pub id: String,
    pub layers: Vec<ResourceImageLayer>,
    /// Internal resource type descriptor
    #[serde(rename = "modelName")]
    pub model_name: ResourceSpriteModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    pub origin: f64,
    #[serde(rename = "originLocked")]
    pub origin_locked: bool,
    #[serde(rename = "playbackSpeed")]
    pub playback_speed: f64,
    #[serde(rename = "playbackSpeedtype")]
    pub playback_speedtype: f64,
    #[serde(rename = "premultiplyAlpha")]
    pub premultiply_alpha: bool,
    pub sepmasks: bool,
    #[serde(rename = "swatchColours")]
    pub swatch_colours: Option<serde_json::Value>,
    #[serde(rename = "swfPrecision")]
    pub swf_precision: f64,
    #[serde(rename = "textureGroupId")]
    pub texture_group_id: String,
    #[serde(rename = "type")]
    pub resource_sprite_type: f64,
    #[serde(rename = "VTile")]
    pub v_tile: bool,
    pub width: f64,
    pub xorig: f64,
    pub yorig: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceFrame {
    #[serde(rename = "compositeImage")]
    pub composite_image: ResourceSpriteImage,
    pub id: String,
    pub images: Vec<ResourceSpriteImage>,
    #[serde(rename = "modelName")]
    pub model_name: ResourceFrameModelName,
    pub mvc: Mvc,
    #[serde(rename = "SpriteID")]
    pub sprite_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceSpriteImage {
    #[serde(rename = "FrameId")]
    pub frame_id: String,
    pub id: String,
    #[serde(rename = "LayerId")]
    pub layer_id: String,
    #[serde(rename = "modelName")]
    pub model_name: ResourceSpriteImageModelName,
    pub mvc: Mvc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceImageLayer {
    #[serde(rename = "blendMode")]
    pub blend_mode: f64,
    pub id: String,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "modelName")]
    pub model_name: ResourceImageLayerModelName,
    pub mvc: String,
    pub name: String,
    pub opacity: f64,
    #[serde(rename = "SpriteId")]
    pub sprite_id: String,
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceSound {
    /// The GUID of the audio group. Unknown where audio group data itself is stored.
    #[serde(rename = "audioGroupGuid")]
    pub audio_group_guid: String,
    /// Quality of the sound, set in the IDE at 8bit or 16bit.
    #[serde(rename = "bitDepth")]
    pub bit_depth: f64,
    /// The Bit Rate in kbps.
    #[serde(rename = "bitRate")]
    pub bit_rate: f64,
    /// Event GUID
    pub id: String,
    /// The "Attribute" of the sound.
    pub kind: f64,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceSoundModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    #[serde(rename = "preLoad")]
    pub pre_load: bool,
    #[serde(rename = "sampleRate")]
    pub sample_rate: f64,
    /// Sound 'type' used in a SoundYY file
    #[serde(rename = "type")]
    pub resource_sound_type: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcePath {
    /// Path closed or open
    pub closed: bool,
    pub hsnap: f64,
    /// Event GUID
    pub id: String,
    /// Straight or smooth path.
    pub kind: f64,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourcePathModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    pub points: Vec<ResourcePathPoint>,
    pub precision: f64,
    pub vsnap: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcePathPoint {
    /// Resource GUID
    pub id: String,
    /// Internal resource type descriptor
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// Version string, unknown use
    pub mvc: String,
    pub speed: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceGmFolder {
    /// An array of the views/resource GUIDs which this folder contains.
    pub children: Vec<String>,
    /// The FilterType of the View
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// The folder name itself
    #[serde(rename = "folderName")]
    pub folder_name: String,
    /// Event GUID
    pub id: String,
    /// Indicates if the view is the Default Node.
    #[serde(rename = "isDefaultView")]
    pub is_default_view: bool,
    /// A code, likely used for adding localizations.
    #[serde(rename = "localisedFolderName")]
    pub localised_folder_name: ResourceLocalisedNames,
    /// Internal resource type descriptor
    #[serde(rename = "modelName")]
    pub model_name: ResourceGmFolderModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceTileset {
    /// Event GUID
    pub id: String,
    /// Internal resource type descriptor
    #[serde(rename = "modelName")]
    pub model_name: ResourceTilesetModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceScript {
    /// Event GUID
    pub id: String,
    #[serde(rename = "IsCompatibility")]
    pub is_compatibility: bool,
    #[serde(rename = "IsDnD")]
    pub is_dn_d: bool,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceScriptModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceFont {
    /// Checks if AntiAliasing is enabled. Not a real boolean, but always 0 or 1.
    #[serde(rename = "AntiAlias")]
    pub anti_alias: f64,
    pub bold: bool,
    pub charset: f64,
    pub first: f64,
    #[serde(rename = "fontName")]
    pub font_name: String,
    pub glyphs: Vec<ResourceGlyph>,
    /// Event GUID
    pub id: String,
    /// Unknown usage.
    pub image: Option<serde_json::Value>,
    #[serde(rename = "includeTTF")]
    pub include_ttf: bool,
    pub italic: bool,
    /// Unknown usage.
    #[serde(rename = "kerningPairs")]
    pub kerning_pairs: Vec<Option<serde_json::Value>>,
    pub last: f64,
    /// Internal resource type descriptor
    #[serde(rename = "modelName")]
    pub model_name: ResourceFontModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    pub ranges: Vec<Range>,
    #[serde(rename = "sampleText")]
    pub sample_text: String,
    pub size: f64,
    #[serde(rename = "styleName")]
    pub style_name: String,
    #[serde(rename = "textureGroupId")]
    pub texture_group_id: String,
    /// Unknown use. Likely related to TTFs
    #[serde(rename = "TTFName")]
    pub ttf_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceGlyph {
    #[serde(rename = "Key")]
    pub key: f64,
    #[serde(rename = "Value")]
    pub value: ResourceGlyphValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceGlyphValue {
    pub character: f64,
    pub h: f64,
    pub id: String,
    #[serde(rename = "modelName")]
    pub model_name: PurpleModelName,
    pub mvc: String,
    pub offset: f64,
    pub shift: f64,
    pub w: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceTimeline {
    /// Event GUID
    pub id: String,
    /// Internal resource type descriptor
    #[serde(rename = "modelName")]
    pub model_name: ResourceTimelineModelName,
    /// Array of "moments" in the timeline
    #[serde(rename = "momentList")]
    pub moment_list: Vec<ResourceMoment>,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceMoment {
    /// Describes the .gml file for each moment. Coded as a Create event.
    /// Yes, this is how it is spelled.
    pub evnt: ResourceObjectEvent,
    #[serde(rename = "modelName")]
    pub model_name: ResourceMomentModelName,
    pub moment: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceNote {
    /// Event GUID
    pub id: String,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceNoteModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceExtension {
    pub files: Vec<ResourceGmExtensionFile>,
    /// Event GUID
    pub id: String,
    /// This is where the extension creator specifies what extra resources are there.
    #[serde(rename = "IncludedResources")]
    pub included_resources: Vec<Option<serde_json::Value>>,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceExtensionModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceGmExtensionFile {
    /// These are the constants, or macros, which the extension provides.
    pub constants: Vec<ResourceGmExtensionConstant>,
    /// Specifies to which targets to compile extension. Bit.
    #[serde(rename = "copyToTargets")]
    pub copy_to_targets: f64,
    /// This is the name of the file which will be in the same folder.
    pub filename: String,
    /// The final function called by the extension.
    #[serde(rename = "final")]
    pub resource_gm_extension_file_final: String,
    /// These are the functions which the extension provides.
    pub functions: Vec<ResourceGmExtensionFunction>,
    /// Event GUID
    pub id: String,
    /// The initial function called.
    pub init: String,
    /// This is the type of the Extension. It is unclear what the types are.
    pub kind: f64,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceGmExtensionFileModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    /// Order of the functions. The strings here refer to the UUIDs of the functions, which is
    /// their ID.
    pub order: Vec<String>,
    /// The original name of the function. Unknown usage.
    pub origname: String,
    /// Array of ProxyFiles. Unknown usage.
    #[serde(rename = "ProxyFiles")]
    pub proxy_files: Vec<Option<serde_json::Value>>,
    /// Whether it is compressed. Unknown usage.
    pub uncompress: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceGmExtensionConstant {
    /// The name of the Macro
    #[serde(rename = "constantName")]
    pub constant_name: String,
    /// If the macro should be hidden from the user
    pub hidden: bool,
    /// Event GUID
    pub id: String,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceGmExtensionConstantModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    /// This is a GML snippet which the Macro will be replaced with. It is identical to #macro
    /// someMacro THIS_IS_THE_SNIPPET.
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceGmExtensionFunction {
    /// The number of arguments which the extension has. Note: -1 indicates that a variable
    /// number of arguments are accepted.
    #[serde(rename = "argCount")]
    pub arg_count: f64,
    /// Array of argument type. See @type ArgumentType for these:
    pub args: Vec<f64>,
    /// The external name of the function.
    #[serde(rename = "externalName")]
    pub external_name: String,
    /// Help is the popup which will come up in GMS2's autocomplete. It is essentially a
    /// signature line.
    pub help: String,
    /// Controls if the function is visible to the user or not.
    pub hidden: bool,
    /// Event GUID
    pub id: String,
    /// This is the type of the Extension, inherited. It is unclear what the types are.
    pub kind: f64,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceGmExtensionFunctionModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// This is the name as the user will see it of the function
    pub name: String,
    /// This is the ArgumentType return.
    #[serde(rename = "returnType")]
    pub return_type: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceShader {
    /// Event GUID
    pub id: String,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceShaderModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    /// Shader language used.
    #[serde(rename = "type")]
    pub resource_shader_type: f64,
}

/// Bare bones interface for now. A Sprite has a lot more
/// to it than just the BaseResource. Will be added soon.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceGmResource {
    /// List of object events
    #[serde(rename = "eventList")]
    pub event_list: Option<Vec<ResourceObjectEvent>>,
    /// Event GUID
    pub id: String,
    /// GUID of sprite mask
    #[serde(rename = "maskSpriteId")]
    pub mask_sprite_id: Option<String>,
    /// Internal resource type descriptor (GMObject)
    ///
    /// Internal resource type descriptor
    ///
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ResourceGmResourceModelName,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    /// Properties overriden.
    #[serde(rename = "overriddenProperties")]
    pub overridden_properties: Option<Vec<HashMap<String, Option<serde_json::Value>>>>,
    /// GUID of parent object
    #[serde(rename = "parentObjectId")]
    pub parent_object_id: Option<String>,
    /// Indicates if this object is persistent
    pub persistent: Option<bool>,
    #[serde(rename = "physicsAngularDamping")]
    pub physics_angular_damping: Option<f64>,
    #[serde(rename = "physicsDensity")]
    pub physics_density: Option<f64>,
    #[serde(rename = "physicsFriction")]
    pub physics_friction: Option<f64>,
    #[serde(rename = "physicsGroup")]
    pub physics_group: Option<f64>,
    #[serde(rename = "physicsKinematic")]
    pub physics_kinematic: Option<bool>,
    #[serde(rename = "physicsLinearDamping")]
    pub physics_linear_damping: Option<f64>,
    /// Indicates if this object uses physics
    #[serde(rename = "physicsObject")]
    pub physics_object: Option<bool>,
    #[serde(rename = "physicsRestitution")]
    pub physics_restitution: Option<f64>,
    #[serde(rename = "physicsSensor")]
    pub physics_sensor: Option<bool>,
    #[serde(rename = "physicsShape")]
    pub physics_shape: Option<f64>,
    #[serde(rename = "physicsShapePoints")]
    pub physics_shape_points: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "physicsStartAwake")]
    pub physics_start_awake: Option<bool>,
    /// Properties are variables set using the "variables" box in the IDE.
    pub properties: Option<Vec<HashMap<String, Option<serde_json::Value>>>>,
    /// Indicates if this object is solid
    pub solid: Option<bool>,
    /// GUID of this object's sprite
    #[serde(rename = "spriteId")]
    pub sprite_id: Option<String>,
    /// Indicates if this object is visible
    pub visible: Option<bool>,
    /// The name of the Room Creation code FP, relative to the Room folder itself.
    #[serde(rename = "creationCodeFile")]
    pub creation_code_file: Option<String>,
    /// Inherit Code from a Parent Room
    #[serde(rename = "inheritCode")]
    pub inherit_code: Option<bool>,
    /// Inherit Creation Order from a Parent Room
    #[serde(rename = "inheritCreationOrder")]
    pub inherit_creation_order: Option<bool>,
    /// Inherit Layers from a Parent Room
    #[serde(rename = "inheritLayers")]
    pub inherit_layers: Option<bool>,
    /// instanceCreationOrderIDs
    #[serde(rename = "instanceCreationOrderIDs")]
    pub instance_creation_order_i_ds: Option<Vec<String>>,
    #[serde(rename = "isDnD")]
    pub resource_gm_resource_is_dn_d: Option<bool>,
    /// All your layers are placed here.
    pub layers: Option<Vec<ResourceELayer>>,
    /// Parent Room ID
    #[serde(rename = "parentID")]
    pub parent_id: Option<String>,
    /// Physics setting of the room.
    #[serde(rename = "physicsSettings")]
    pub physics_settings: Option<ResourcePhysicsSettings>,
    pub bbox_bottom: Option<f64>,
    pub bbox_left: Option<f64>,
    pub bbox_right: Option<f64>,
    pub bbox_top: Option<f64>,
    pub bboxmode: Option<f64>,
    pub colkind: Option<f64>,
    pub coltolerance: Option<f64>,
    #[serde(rename = "edgeFiltering")]
    pub edge_filtering: Option<bool>,
    #[serde(rename = "For3D")]
    pub for3_d: Option<bool>,
    pub frames: Option<Vec<ResourceFrame>>,
    #[serde(rename = "gridX")]
    pub grid_x: Option<f64>,
    #[serde(rename = "gridY")]
    pub grid_y: Option<f64>,
    pub height: Option<f64>,
    #[serde(rename = "HTile")]
    pub h_tile: Option<bool>,
    pub origin: Option<f64>,
    #[serde(rename = "originLocked")]
    pub origin_locked: Option<bool>,
    #[serde(rename = "playbackSpeed")]
    pub playback_speed: Option<f64>,
    #[serde(rename = "playbackSpeedtype")]
    pub playback_speedtype: Option<f64>,
    #[serde(rename = "premultiplyAlpha")]
    pub premultiply_alpha: Option<bool>,
    pub sepmasks: Option<bool>,
    #[serde(rename = "swatchColours")]
    pub swatch_colours: Option<serde_json::Value>,
    #[serde(rename = "swfPrecision")]
    pub swf_precision: Option<f64>,
    #[serde(rename = "textureGroupId")]
    pub texture_group_id: Option<String>,
    /// Sound 'type' used in a SoundYY file
    ///
    /// Shader language used.
    #[serde(rename = "type")]
    pub resource_gm_resource_type: Option<f64>,
    #[serde(rename = "VTile")]
    pub v_tile: Option<bool>,
    pub width: Option<f64>,
    pub xorig: Option<f64>,
    pub yorig: Option<f64>,
    /// The GUID of the audio group. Unknown where audio group data itself is stored.
    #[serde(rename = "audioGroupGuid")]
    pub audio_group_guid: Option<String>,
    /// Quality of the sound, set in the IDE at 8bit or 16bit.
    #[serde(rename = "bitDepth")]
    pub bit_depth: Option<f64>,
    /// The Bit Rate in kbps.
    #[serde(rename = "bitRate")]
    pub bit_rate: Option<f64>,
    /// The "Attribute" of the sound.
    ///
    /// Straight or smooth path.
    pub kind: Option<f64>,
    #[serde(rename = "preLoad")]
    pub pre_load: Option<bool>,
    #[serde(rename = "sampleRate")]
    pub sample_rate: Option<f64>,
    pub volume: Option<f64>,
    /// Path closed or open
    pub closed: Option<bool>,
    pub hsnap: Option<f64>,
    pub points: Option<Vec<ResourcePathPoint>>,
    pub precision: Option<f64>,
    pub vsnap: Option<f64>,
    /// An array of the views/resource GUIDs which this folder contains.
    pub children: Option<Vec<String>>,
    /// The FilterType of the View
    #[serde(rename = "filterType")]
    pub filter_type: Option<String>,
    /// The folder name itself
    #[serde(rename = "folderName")]
    pub folder_name: Option<String>,
    /// Indicates if the view is the Default Node.
    #[serde(rename = "isDefaultView")]
    pub is_default_view: Option<bool>,
    /// A code, likely used for adding localizations.
    #[serde(rename = "localisedFolderName")]
    pub localised_folder_name: Option<ResourceLocalisedNames>,
    #[serde(rename = "IsCompatibility")]
    pub is_compatibility: Option<bool>,
    #[serde(rename = "IsDnD")]
    pub is_dn_d: Option<bool>,
    /// Checks if AntiAliasing is enabled. Not a real boolean, but always 0 or 1.
    #[serde(rename = "AntiAlias")]
    pub anti_alias: Option<f64>,
    pub bold: Option<bool>,
    pub charset: Option<f64>,
    pub first: Option<f64>,
    #[serde(rename = "fontName")]
    pub font_name: Option<String>,
    pub glyphs: Option<Vec<ResourceGlyph>>,
    /// Unknown usage.
    pub image: Option<serde_json::Value>,
    #[serde(rename = "includeTTF")]
    pub include_ttf: Option<bool>,
    pub italic: Option<bool>,
    /// Unknown usage.
    #[serde(rename = "kerningPairs")]
    pub kerning_pairs: Option<Vec<Option<serde_json::Value>>>,
    pub last: Option<f64>,
    pub ranges: Option<Vec<Range>>,
    #[serde(rename = "sampleText")]
    pub sample_text: Option<String>,
    pub size: Option<f64>,
    #[serde(rename = "styleName")]
    pub style_name: Option<String>,
    /// Unknown use. Likely related to TTFs
    #[serde(rename = "TTFName")]
    pub ttf_name: Option<String>,
    /// Array of "moments" in the timeline
    #[serde(rename = "momentList")]
    pub moment_list: Option<Vec<ResourceMoment>>,
    pub files: Option<Vec<ResourceGmExtensionFile>>,
    /// This is where the extension creator specifies what extra resources are there.
    #[serde(rename = "IncludedResources")]
    pub included_resources: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceELayer {
    #[serde(rename = "__type")]
    pub resource_e_layer_type: Option<String>,
    pub depth: Option<f64>,
    pub grid_x: Option<f64>,
    pub grid_y: Option<f64>,
    #[serde(rename = "hierarchyFrozen")]
    pub hierarchy_frozen: Option<bool>,
    #[serde(rename = "hierarchyVisible")]
    pub hierarchy_visible: Option<bool>,
    /// Event GUID
    pub id: String,
    #[serde(rename = "inheritLayerDepth")]
    pub inherit_layer_depth: Option<bool>,
    #[serde(rename = "inheritLayerSettings")]
    pub inherit_layer_settings: Option<bool>,
    #[serde(rename = "inheritSubLayers")]
    pub inherit_sub_layers: Option<bool>,
    #[serde(rename = "inheritVisibility")]
    pub inherit_visibility: Option<bool>,
    pub instances: Option<Vec<ResourceInstance>>,
    pub layers: Option<Vec<ResourceLayer>>,
    #[serde(rename = "m_parentID")]
    pub m_parent_id: Option<String>,
    #[serde(rename = "m_serialiseFrozen")]
    pub m_serialise_frozen: Option<bool>,
    /// Internal resource type descriptor (GMEvent)
    #[serde(rename = "modelName")]
    pub model_name: ModelNameEnum,
    /// Version string, unknown use
    pub mvc: String,
    /// Resource name
    pub name: String,
    pub userdefined_depth: Option<bool>,
    pub visible: bool,
    #[serde(rename = "blendMode")]
    pub blend_mode: Option<f64>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
    pub opacity: Option<f64>,
    #[serde(rename = "SpriteId")]
    pub sprite_id: Option<String>,
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
    pub resource_type: ResourceModelNames,
}

/// Internal resource type descriptor (GMEvent)
///
/// Describes the resource type
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceModelNames {
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

/// Internal resource type descriptor (GMObject)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceObjectModelName {
    #[serde(rename = "GMObject")]
    GmObject,
}

/// Internal resource type descriptor
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceOptionsModelName {
    #[serde(rename = "GMOption")]
    GmOption,
}

/// Internal resource type descriptor
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceRoomModelName {
    #[serde(rename = "GMRoom")]
    GmRoom,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceSpriteImageModelName {
    #[serde(rename = "GMSpriteImage")]
    GmSpriteImage,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mvc {
    #[serde(rename = "1.0")]
    The10,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceFrameModelName {
    #[serde(rename = "GMSpriteFrame")]
    GmSpriteFrame,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceImageLayerModelName {
    #[serde(rename = "GMImageLayer")]
    GmImageLayer,
}

/// Internal resource type descriptor
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceSpriteModelName {
    #[serde(rename = "GMSprite")]
    GmSprite,
}

/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceSoundModelName {
    #[serde(rename = "GMSound")]
    GmSound,
}

/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourcePathModelName {
    #[serde(rename = "GMPath")]
    GmPath,
}

/// A code, likely used for adding localizations.
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceLocalisedNames {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "ResourceTree_Configs")]
    ResourceTreeConfigs,
    #[serde(rename = "ResourceTree_Extensions")]
    ResourceTreeExtensions,
    #[serde(rename = "ResourceTree_Fonts")]
    ResourceTreeFonts,
    #[serde(rename = "ResourceTree_IncludedFiles")]
    ResourceTreeIncludedFiles,
    #[serde(rename = "ResourceTree_Notes")]
    ResourceTreeNotes,
    #[serde(rename = "ResourceTree_Objects")]
    ResourceTreeObjects,
    #[serde(rename = "ResourceTree_Options")]
    ResourceTreeOptions,
    #[serde(rename = "ResourceTree_Paths")]
    ResourceTreePaths,
    #[serde(rename = "ResourceTree_Rooms")]
    ResourceTreeRooms,
    #[serde(rename = "ResourceTree_Scripts")]
    ResourceTreeScripts,
    #[serde(rename = "ResourceTree_Shaders")]
    ResourceTreeShaders,
    #[serde(rename = "ResourceTree_Sounds")]
    ResourceTreeSounds,
    #[serde(rename = "ResourceTree_Sprites")]
    ResourceTreeSprites,
    #[serde(rename = "ResourceTree_Tilesets")]
    ResourceTreeTilesets,
    #[serde(rename = "ResourceTree_Timelines")]
    ResourceTreeTimelines,
}

/// Internal resource type descriptor
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceGmFolderModelName {
    #[serde(rename = "GMFolder")]
    GmFolder,
}

/// Internal resource type descriptor
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceTilesetModelName {
    #[serde(rename = "GMTileSet")]
    GmTileSet,
}

/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceScriptModelName {
    #[serde(rename = "GMScript")]
    GmScript,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PurpleModelName {
    #[serde(rename = "GMGlyph")]
    GmGlyph,
}

/// Internal resource type descriptor
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceFontModelName {
    #[serde(rename = "GMFont")]
    GmFont,
}

/// Internal resource type descriptor
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceTimelineModelName {
    #[serde(rename = "GMTimeline")]
    GmTimeline,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceMomentModelName {
    #[serde(rename = "GMMoment")]
    GmMoment,
}

/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceNoteModelName {
    #[serde(rename = "GMNote")]
    GmNote,
}

/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceGmExtensionConstantModelName {
    #[serde(rename = "GMExtensionConstant")]
    GmExtensionConstant,
}

/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceGmExtensionFunctionModelName {
    #[serde(rename = "GMExtensionFunction")]
    GmExtensionFunction,
}

/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceGmExtensionFileModelName {
    #[serde(rename = "GMExtensionFile")]
    GmExtensionFile,
}

/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceExtensionModelName {
    #[serde(rename = "GMExtension")]
    GmExtension,
}

/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceShaderModelName {
    #[serde(rename = "GMShader")]
    GmShader,
}

/// Internal resource type descriptor (GMEvent)
///
/// Describes the resource type
#[derive(Debug, Serialize, Deserialize)]
pub enum ModelNameEnum {
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
    #[serde(rename = "GMImageLayer")]
    GmImageLayer,
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

/// Internal resource type descriptor (GMObject)
///
/// Internal resource type descriptor
///
/// Internal resource type descriptor (GMEvent)
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceGmResourceModelName {
    #[serde(rename = "GMExtension")]
    GmExtension,
    #[serde(rename = "GMFolder")]
    GmFolder,
    #[serde(rename = "GMFont")]
    GmFont,
    #[serde(rename = "GMNote")]
    GmNote,
    #[serde(rename = "GMObject")]
    GmObject,
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

/// Usually contains resource type, in this case GMProject
#[derive(Debug, Serialize, Deserialize)]
pub enum YypModelName {
    #[serde(rename = "GMProject")]
    GmProject,
}

/// Describes object entry type, which is always "GMResourceInfo" for YYPResources
#[derive(Debug, Serialize, Deserialize)]
pub enum FluffyModelName {
    #[serde(rename = "GMResourceInfo")]
    GmResourceInfo,
}

/// Describes object entry type.
#[derive(Debug, Serialize, Deserialize)]
pub enum ParentProjectModelName {
    #[serde(rename = "GMProjectParent")]
    GmProjectParent,
}
