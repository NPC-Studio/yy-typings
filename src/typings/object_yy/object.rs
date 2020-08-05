use super::{ConstGmEvent, ConstGmObject, ConstGmObjectProperty, EventType};
use crate::{FilesystemPath, ResourceVersion, Tags, ViewPath};
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
    pub physics_sensor: bool,
    pub physics_shape: usize, // @todo
    pub physics_group: usize, // @todo
    pub physics_density: f64,
    pub physics_restitution: f64,
    pub physics_linear_damping: f64,
    pub physics_angular_damping: f64,
    pub physics_friction: f64,
    pub physics_start_awake: bool,
    pub physics_kinematic: bool,
    pub physics_shape_points: Vec<()>, // @todo

    // Event list and Properties
    pub event_list: Vec<ObjectEvent>,

    /// The properties which were made in this object directly.
    pub properties: Vec<ObjectProperty>,
    /// The properties which were made in a parent object AND overriden. If the parent object's properties
    /// have not been overriden, then they will not appear anywhere in this object's `yy` files and must
    /// be found recursively.
    pub overridden_properties: Vec<ObjectProperty>,

    // View Data
    /// The parent in the Gms2 virtual file system, ie. the parent which
    /// a user would see in the Navigation Pane in Gms2. This has no relationship
    /// to the actual operating system's filesystem.
    pub parent: ViewPath,
    /// The resource version of this yy file. At default 1.0.
    pub resource_version: ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: String,

    /// The tags given to the object.
    pub tags: Tags,

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
    /// Filesystem path pointing directly to the parent Object,
    /// such as:
    /// ```json
    /// {
    ///     "name": "obj_stairs",
    ///     "path": "objects/obj_stairs/obj_stairs.yy"
    /// }
    /// ```
    pub parent: FilesystemPath,

    /// The version of the `.yy` file.
    pub resource_version: ResourceVersion,

    /// The "name" of the Event, which appears to always be null or an empty string
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub name: Option<String>,

    /// The tags for the event, which probably should always be empty.
    pub tags: Tags,

    /// The constant resource type for GmEvents.
    pub resource_type: ConstGmEvent,
}

/// Object "properties" are set in the Gms2 window and allow the user to override those properties either
/// in child objects of a parent, or in the Room (or both!). This allows for simple customization in the room editor.
#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ObjectProperty {
    /// The type of property which is preset. Some, or all, of the rest of the information
    /// in this struct will be used based on the property type.
    pub var_type: ObjectPropertyTypes,
    /// The serialized value of the property type. This corresponds exactly to what the Gms2 box
    /// will have inside it as a string.
    pub value: String,
    /// If the range Ui option is enabled for this type. This is ignored unless `var_type` is `Real` or `Integer`.
    pub range_enabled: bool,
    /// The minimum range. Minimin should be less than max, but does not error if so.
    pub range_min: f64,
    /// The maximum range. Minimin should be less than max, but does not error if so.
    pub range_max: f64,
    /// The items which can be selected when `var_type` is set to `List`. Ignored in any other `var_type`.
    pub list_items: Vec<String>,
    /// If set to true when `var_type` is set to `List`, allows the User to select multiple options.
    pub multiselect: bool,
    /// Not sure what this is supposed to be. In the meantime, we've typed it as a blank array.
    pub filters: [(); 0],
    /// The ResourceVersion, default value.
    pub resource_version: ResourceVersion,
    /// The name of the property, such as "room_to_transition_to".
    pub name: String,
    /// The tags assigned to the property. Probably shouldn't be assigned.
    pub tags: Tags,
    /// The resource type const of the property.
    pub resource_type: ConstGmObjectProperty,
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

#[cfg(test)]
mod tests {
    use crate::{object_yy::*, utils::TrailingCommaUtility, ViewPathLocation};
    use include_dir::{include_dir, Dir, DirEntry};
    use pretty_assertions::assert_eq;

    #[test]
    fn trivial_sprite_parsing() {
        let all_objects: Dir = include_dir!("data/objects");
        let tcu = TrailingCommaUtility::new();

        for object_file in all_objects.find("**/*.yy").unwrap() {
            if let DirEntry::File(file) = object_file {
                println!("parsing {}", file.path);
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
            sprite_mask_id: None,
            persistent: false,
            parent_object_id: None,
            physics_object: false,
            physics_sensor: false,
            physics_shape: 1,
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
                parent: FilesystemPath::new("objects", "obj_animate_then_die"),
                resource_version: ResourceVersion::default(),
                name: None,
                tags: vec![],
                resource_type: ConstGmEvent::Const,
            }],
            properties: vec![],
            overridden_properties: vec![],
            parent: ViewPath {
                name: "ui".to_string(),
                path: ViewPathLocation("folders/Objects/ui.yy".to_owned()),
            },
            resource_version: ResourceVersion::default(),
            name: "obj_animate_then_die".to_string(),
            tags: vec![],
            resource_type: ConstGmObject::Const,
        };

        assert_eq!(parsed_object, object);
    }
}
