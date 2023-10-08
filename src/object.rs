mod event_type;
pub use event_type::*;

mod vk;
pub use vk::*;

use crate::FilesystemPath;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    /// The common data for all yy resources.
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::Object>,

    // Event list and Properties
    pub event_list: Vec<ObjectEvent>,

    /// Unclear what it means, always set to `true` in practice.
    pub managed: bool,

    /// The properties which were made in a parent object AND overriden. If the
    /// parent object's properties have not been overriden, then they will
    /// not appear anywhere in this object's `yy` files and must
    /// be found recursively.
    pub overridden_properties: Vec<ObjectOverrideProperty>,

    /// The parent in the Gms2 virtual file system, ie. the parent which
    /// a user would see in the Navigation Pane in Gms2. This has no
    /// relationship to the actual operating system's filesystem.
    pub parent: crate::ViewPath,

    /// The id of the parent object for the Inhertance in Gms2.
    pub parent_object_id: Option<FilesystemPath>,

    /// If the object is "persistent", meaning if Gms2 will keep the object
    /// between room change events.
    pub persistent: bool,

    /// The angular damping set.
    pub physics_angular_damping: f64,

    /// The density.
    pub physics_density: f64,

    /// The friction set.
    pub physics_friction: f64,

    /// What numerical group it belongs to. 0 is a special non-group value.
    pub physics_group: usize,

    /// Whether this physics object is kinematic or not.
    pub physics_kinematic: bool,

    /// The linear damping.
    pub physics_linear_damping: f64,

    // Physics
    /// Is this a physics object?
    pub physics_object: bool,

    /// The restitution.
    pub physics_restitution: f64,

    /// Enabled if the objects is a physics sensor.
    pub physics_sensor: bool,

    /// The shape of the physics, which is needed to understand the shape
    /// points.
    pub physics_shape: PhysicsShape,

    /// The shape points of the physics shape.
    pub physics_shape_points: Vec<PhysicsVec2>,

    /// Whether this object should start awake or not.
    pub physics_start_awake: bool,

    /// The properties which were made in this object directly.
    pub properties: Vec<ObjectProperty>,

    /// If the object is marked as solid for the collision system.
    pub solid: bool,

    /// The Id of the Sprite being used for this object.
    pub sprite_id: Option<FilesystemPath>,

    /// The Id used for the Collision Mask, if not the SpriteId.
    pub sprite_mask_id: Option<FilesystemPath>,

    /// The tags associated with this object.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub tags: Vec<String>,

    /// If the object is visible.
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ObjectEvent {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::Event>,

    /// The Id of the thing to collide with.
    pub collision_object_id: Option<FilesystemPath>,

    /// The type of the event. In the JSON, this is represented with two enums,
    /// but we use Serde to succesfully parse this into idiomatic Rust enums.
    #[serde(flatten)]
    pub event_type: EventType,

    /// Is this event used in DragNDrop, the thing no one uses?
    pub is_dn_d: bool,
}

/// Object "properties" are set in the Gms2 window and allow the user to
/// override those properties either in child objects of a parent, or in the
/// Room (or both!). This allows for simple customization in the room editor.
#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ObjectProperty {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::ObjectProperty>,

    /// Not sure what this is supposed to be. In the meantime, we've typed it as
    /// a blank array.
    pub filters: Vec<String>,

    /// The items which can be selected when `var_type` is set to `List`.
    /// Ignored in any other `var_type`.
    pub list_items: Vec<String>,

    /// If set to true when `var_type` is set to `List`, allows the User to
    /// select multiple options.
    pub multiselect: bool,

    /// If the range Ui option is enabled for this type. This is ignored unless
    /// `var_type` is `Real` or `Integer`.
    pub range_enabled: bool,

    /// The maximum range. Minimin should be less than max, but does not error
    /// if so.
    pub range_max: f64,

    /// The minimum range. Minimin should be less than max, but does not error
    /// if so.
    pub range_min: f64,

    /// The serialized value of the property type. This corresponds exactly to
    /// what the Gms2 box will have inside it as a string.
    pub value: String,

    /// The type of property which is preset. Some, or all, of the rest of the
    /// information in this struct will be used based on the property type.
    pub var_type: ObjectPropertyTypes,
}

/// Object "properties" are set in the Gms2 window and allow the user to
/// override those properties either in child objects of a parent, or in the
/// Room (or both!). This allows for simple customization in the room editor.
#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ObjectOverrideProperty {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::ObjectOverrideProperty>,

    /// This is **not** a real filesystem path, but instead just looks like one.
    /// Eventually, this will receive better typing. @todo
    /// The `name` is the name of the prperty, and the `path` is to the
    /// ORIGINATOR of the property.
    pub property_id: Option<FilesystemPath>,

    /// The path to the object which this property last overrides.
    pub object_id: FilesystemPath,

    /// The serialized value of the property type. This corresponds exactly to
    /// what the Gms2 box will have inside it as a string.
    pub value: String,
}

/// The types of object "Properties" as set in the Gms2 Widget pane by users.
#[derive(
    Debug,
    Serialize_repr,
    Deserialize_repr,
    SmartDefault,
    PartialEq,
    Clone,
    Copy,
    Eq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum ObjectPropertyTypes {
    #[default]
    Real,
    Integer,
    String,
    Boolean,
    Expression,
    Asset,
    List,
    Colour,
}

/// The types of physics object as specified in the Gms2 editor.
#[derive(
    Debug,
    Serialize_repr,
    Deserialize_repr,
    SmartDefault,
    PartialEq,
    Clone,
    Copy,
    Eq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum PhysicsShape {
    Circle,
    #[default]
    Box,
    ConvexShape,
}

/// The types of physics object as specified in the Gms2 editor.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Copy, PartialOrd)]
pub struct PhysicsVec2 {
    pub x: f32,
    pub y: f32,
}

gm_const!(
    Object -> "GMObject",
    Event -> "GMEvent",
    ObjectProperty -> "GMObjectProperty",
    ObjectOverrideProperty -> "GMOverriddenProperty",
);
