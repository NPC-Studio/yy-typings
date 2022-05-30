use super::{
    ConstGmEvent, ConstGmObject, ConstGmObjectOverrideProperty, ConstGmObjectProperty, EventType,
};
use crate::{FilesystemPath, ResourceData, ResourceVersion, Tags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    // Ids:
    /// The Id of the Sprite being used for this object.
    pub sprite_id: Option<FilesystemPath>,

    /// If the object is marked as solid for the collision system.
    pub solid: bool,
    /// If the object is visible.
    pub visible: bool,
    /// Unclear what it means, always set to `true` in practice.
    pub managed: bool,
    /// The Id used for the Collision Mask, if not the SpriteId.
    pub sprite_mask_id: Option<FilesystemPath>,
    /// If the object is "persistent", meaning if Gms2 will keep the object
    /// between room change events.
    pub persistent: bool,
    /// The id of the parent object for the Inhertance in Gms2.
    pub parent_object_id: Option<FilesystemPath>,

    // Physics
    /// Is this a physics object?
    pub physics_object: bool,

    /// Enabled if the objects is a physics sensor.
    pub physics_sensor: bool,

    /// The shape of the physics, which is needed to understand the shape
    /// points.
    pub physics_shape: PhysicsShape,

    /// What numerical group it belongs to. 0 is a special non-group value.
    pub physics_group: usize,

    /// The density.
    pub physics_density: f64,

    /// The restitution.
    pub physics_restitution: f64,

    /// The linear damping.
    pub physics_linear_damping: f64,

    /// The angular damping set.
    pub physics_angular_damping: f64,

    /// The friction set.
    pub physics_friction: f64,

    /// Whether this object should start awake or not.
    pub physics_start_awake: bool,

    /// Whether this physics object is kinematic or not.
    pub physics_kinematic: bool,

    /// The shape points of the physics shape.
    pub physics_shape_points: Vec<PhysicsVec2>,

    // Event list and Properties
    pub event_list: Vec<ObjectEvent>,

    /// The properties which were made in this object directly.
    pub properties: Vec<ObjectProperty>,
    /// The properties which were made in a parent object AND overriden. If the
    /// parent object's properties have not been overriden, then they will
    /// not appear anywhere in this object's `yy` files and must
    /// be found recursively.
    pub overridden_properties: Vec<ObjectOverrideProperty>,

    /// Common resource data.
    #[serde(flatten)]
    pub resource_data: ResourceData,

    /// Const id tag of the object, given by Gms2.
    pub resource_type: ConstGmObject,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ObjectEvent {
    /// Is this event used in DragNDrop, the thing no one uses?
    pub is_dn_d: bool,

    /// The type of the event. In the JSON, this is represented with two enums,
    /// but we use Serde to succesfully parse this into idiomatic Rust enums.
    #[serde(flatten)]
    pub event_type: EventType,

    /// The Id of the thing to collide with.
    pub collision_object_id: Option<FilesystemPath>,

    /// The version of the `.yy` file.
    pub resource_version: ResourceVersion,

    /// The "name" of the Event, which appears to always be null or an empty
    /// string
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub name: Option<String>,

    /// The tags for the event, which probably should always be empty.
    pub tags: Tags,

    /// The constant resource type for GmEvents.
    pub resource_type: ConstGmEvent,
}

/// Object "properties" are set in the Gms2 window and allow the user to
/// override those properties either in child objects of a parent, or in the
/// Room (or both!). This allows for simple customization in the room editor.
#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ObjectProperty {
    /// The type of property which is preset. Some, or all, of the rest of the
    /// information in this struct will be used based on the property type.
    pub var_type: ObjectPropertyTypes,
    /// The serialized value of the property type. This corresponds exactly to
    /// what the Gms2 box will have inside it as a string.
    pub value: String,
    /// If the range Ui option is enabled for this type. This is ignored unless
    /// `var_type` is `Real` or `Integer`.
    pub range_enabled: bool,
    /// The minimum range. Minimin should be less than max, but does not error
    /// if so.
    pub range_min: f64,
    /// The maximum range. Minimin should be less than max, but does not error
    /// if so.
    pub range_max: f64,
    /// The items which can be selected when `var_type` is set to `List`.
    /// Ignored in any other `var_type`.
    pub list_items: Vec<String>,
    /// If set to true when `var_type` is set to `List`, allows the User to
    /// select multiple options.
    pub multiselect: bool,
    /// Not sure what this is supposed to be. In the meantime, we've typed it as
    /// a blank array.
    pub filters: Vec<String>,
    /// The ResourceVersion, default value.
    pub resource_version: ResourceVersion,
    /// The name of the property, such as "room_to_transition_to".
    pub name: String,
    /// The tags assigned to the property. Probably shouldn't be assigned.
    pub tags: Tags,
    /// The resource type const of the property.
    pub resource_type: ConstGmObjectProperty,
}

/// Object "properties" are set in the Gms2 window and allow the user to
/// override those properties either in child objects of a parent, or in the
/// Room (or both!). This allows for simple customization in the room editor.
#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ObjectOverrideProperty {
    /// This is **not** a real filesystem path, but instead just looks like one.
    /// Eventually, this will receive better typing. @todo
    /// The `name` is the name of the prperty, and the `path` is to the
    /// ORIGINATOR of the property.
    pub property_id: FilesystemPath,

    /// The path to the object which this property last overrides.
    pub object_id: FilesystemPath,

    /// The serialized value of the property type. This corresponds exactly to
    /// what the Gms2 box will have inside it as a string.
    pub value: String,

    /// The resource version for this property override
    pub resource_version: ResourceVersion,

    /// The name of the property, which appears to **always** be an empty
    /// string.
    // #[serde(
    //     serialize_with = "ser_nullable_string",
    //     deserialize_with = "de_nullable_string"
    // )]
    pub name: Option<String>,

    /// The tags assigned to the property. Probably shouldn't be assigned.
    pub tags: Tags,

    /// The resource type const of the property.
    pub resource_type: ConstGmObjectOverrideProperty,
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

#[cfg(test)]
mod tests {
    use crate::{object_yy::*, utils::TrailingCommaUtility, ViewPathLocation};
    use include_dir::{include_dir, Dir, DirEntry};
    use pretty_assertions::assert_eq;

    static ALL_OBJECTS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data/objects");

    #[test]
    fn trivial_sprite_parsing() {
        let tcu = TrailingCommaUtility::new();

        for object_file in ALL_OBJECTS.find("**/*.yy").unwrap() {
            if let DirEntry::File(file) = object_file {
                println!("parsing {}", file.path().display());
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                serde_json::from_str::<Object>(&our_str).unwrap();
            }
        }
    }

    #[test]
    fn deep_equality() {
        let object1 = include_str!("../../../data/objects/obj_animate_then_die.yy");

        let parsed_object: Object =
            serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(object1))
                .unwrap();

        let object = Object {
            sprite_id: None,
            solid: false,
            visible: true,
            managed: true,
            sprite_mask_id: None,
            persistent: false,
            parent_object_id: None,
            physics_object: false,
            physics_sensor: false,
            physics_shape: PhysicsShape::Box,
            physics_group: 1,
            physics_density: 0.5,
            physics_restitution: 0.1,
            physics_linear_damping: 0.1,
            physics_angular_damping: 0.1,
            physics_friction: 0.2,
            physics_start_awake: true,
            physics_kinematic: false,
            physics_shape_points: vec![],
            event_list: vec![ObjectEvent {
                is_dn_d: false,
                event_type: EventType::Other(OtherEvent::AnimationEnd),
                collision_object_id: None,
                resource_version: ResourceVersion::default(),
                name: None,
                tags: vec![],
                resource_type: ConstGmEvent::Const,
            }],
            properties: vec![],
            overridden_properties: vec![],
            resource_data: ResourceData {
                parent: ViewPath {
                    name: "ui".to_string(),
                    path: ViewPathLocation("folders/Objects/ui.yy".to_owned()),
                },
                resource_version: ResourceVersion::default(),
                name: "obj_animate_then_die".to_string(),
                tags: vec![],
            },
            resource_type: ConstGmObject::Const,
        };

        assert_eq!(parsed_object, object);
    }
}
