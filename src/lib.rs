#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms)]
#![warn(clippy::dbg_macro)]
#![cfg_attr(not(test), warn(clippy::print_stdout))]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::todo)]
#![deny(rustdoc::broken_intra_doc_links)]

macro_rules! create_guarded_uuid {
    ($this_val:ident) => {
        /// A newtype wrapper around a `uuid::Uuid`. The inner value can always be
        /// accessed with `inner` without consuming the wrapper -- its purpose is for
        /// developer simplicity.
        #[derive(PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Copy, Clone)]
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

        impl Default for $this_val {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

macro_rules! gm_const {
    ($($struct_name:ident -> $serde_name:literal),+ $(,)?) => {
        mod consts {
            $(
                #[derive(
                    Debug,
                    serde::Serialize,
                    serde::Deserialize,
                    PartialEq,
                    Eq,
                    Hash,
                    Clone,
                    Copy,
                    Default,
                    PartialOrd,
                    Ord
                )]
                pub enum $struct_name {
                    #[serde(rename = $serde_name)]
                    #[default]
                    Const,
                }
            )+
        }
    };
}

mod paths;
pub use paths::*;

mod tileset;
pub use tileset::*;

mod audio_group;
pub use audio_group::AudioGroup;

mod sprite;
pub use sprite::*;

mod object;
pub use object::*;

/// Typings associated with Texture Groups.
mod texture_group;
pub use texture_group::*;

/// Typings for Scripts.
mod script;
pub use script::*;

/// Typings for Shaders.
mod shader;
pub use shader::*;

mod sound;
pub use sound::*;

// pub mod room;
// pub use room::*;

mod resource_version;
pub use resource_version::ResourceVersion;

mod yyp;
pub use yyp::*;

mod unidentified_resource;
pub use unidentified_resource::*;

mod note;
pub use note::Note;

mod resource_data;
pub use resource_data::CommonData;

mod utils;
pub use utils::{ResourceNameValidator, TrailingCommaUtility};
