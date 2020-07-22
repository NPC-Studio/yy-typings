#![allow(clippy::bool_comparison)]

//! This is a library was created for the development of [Fields of Mistria](https://twitter.com/FieldsofMistria), a farming RPG with *tons* of Sprites, by NPC Studio. This tool was created to support an Aseprite -> GMS2 pipeline tool. That tool is not public. Using this tool, one should be able to generate their own pipeline without difficulty.
//!
//! ***This crate only supports Gms2, and only supports Gms2 2.3 and above***. If users do want to use a version with Gms2 version 2.2, there is a historical release on the main branch which was made before 2.3's release, though it is not nearly as fully featured as the current branch.
//!
//! This repository has a pair: [the Yy-Boss](https://crates.io/crates/yy-boss), which provides active Yyp handling over stdin/stdout, abstracting over Gms2's native types to allow users to dynamically create resources (and analyze existing resources) without handling the Gms2 Yy files directly.
//!

macro_rules! create_guarded_uuid {
    ($this_val:ident) => {
        /// A newtype wrapper around a `uuid::Uuid`. The inner value can always be accessed
        /// with `inner` without consuming the wrapper -- its purpose is for developer simplicity.
        #[derive(
            PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Copy, Clone, Default,
        )]
        pub struct $this_val(uuid::Uuid);

        impl std::fmt::Debug for $this_val {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} -- {}", stringify!($this_val), self.0)
            }
        }

        impl $this_val {
            /// Creates a new Id using `Uuid::new_v4` which is randomly generated.
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            /// Creates a new Id with the provided Uuid.
            pub fn with_id(id: uuid::Uuid) -> Self {
                Self(id)
            }

            /// Creates a new Id with the provided String which *must* be a Uuid string.
            /// This does an unwrap internally, so probably don't use it!
            pub fn with_string(input: &str) -> Self {
                Self(uuid::Uuid::parse_str(input).unwrap())
            }

            /// Gives access to the inner ID. Try to not use this one too much!
            pub fn inner(&self) -> uuid::Uuid {
                self.0
            }
        }
    };
}

mod typings {
    mod paths;
    pub use paths::*;

    mod tags;
    pub use tags::Tags;

    mod audio_group;
    pub use audio_group::AudioGroup;

    /// Typings associated with Sprite `.yy` files, including
    /// many of the types associated with `Sequences`, for now. In future
    /// iterations of this crate, those shared resources will be in their own module.
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

    /// Typings associated with Object `.yy` files, including typing for Events.
    pub mod object_yy {
        pub use super::*;

        mod object;
        pub use object::*;

        mod object_constants;
        pub use object_constants::*;

        mod event_type;
        pub use event_type::*;
    }

    /// Typings associated with Texture Groups.
    pub mod texture_group;

    mod resource_version;
    pub use resource_version::ResourceVersion;

    mod yyp;
    pub use yyp::*;
}
pub use typings::*;

/// Two utilities which may be useful for downstream crates:
///
/// 1.  `TrailingCommaUtility` will *remove* all trailing commas
///     from a given input string. It is a wrapper over a Regex pattern.
/// 2.  `PathValidator` will validate any paths as valid Gms2 names for a resource.
pub mod utils {
    mod trailing_comma_utility;
    pub use trailing_comma_utility::TrailingCommaUtility;

    mod path_validator;
    pub use path_validator::PathValidator;
}
