mod paths;
pub use paths::*;

mod tileset;
pub use tileset::TileSet;

mod tags;
pub use tags::Tags;

mod audio_group;
pub use audio_group::AudioGroup;

/// Typings associated with Sprite `.yy` files, including
/// many of the types associated with `Sequences`, for now. In future
/// iterations of this crate, those shared resources will be in their own
/// module.
pub mod sprite_yy {
    pub use super::*;

    mod sprite;
    pub use sprite::*;

    mod sprite_constants;
    pub use sprite_constants::*;

    mod sequence;
    pub use sequence::*;

    mod frames_layers;
    pub use frames_layers::*;
}

/// Typings associated with Object `.yy` files, including typing for Events
/// and Vk Keycodes.
pub mod object_yy {
    pub use super::*;

    mod object;
    pub use object::*;

    mod object_constants;
    pub use object_constants::*;

    mod event_type;
    pub use event_type::*;

    mod vk;
    pub use vk::*;
}

/// Typings associated with Texture Groups.
pub mod texture_group;

/// Typings for Scripts.
pub mod script;

/// Typings for Shaders.
pub mod shader;

/// Typings for Sounds.
pub mod sound;

/// Typings for Rooms.
pub mod room;
pub use room::Room;

mod resource_version;
pub use resource_version::ResourceVersion;

mod yyp;
pub use yyp::*;

mod unidentified_resource;
pub use unidentified_resource::*;

mod note;
pub use note::Note;

mod resource_data;
pub use resource_data::ResourceData;
