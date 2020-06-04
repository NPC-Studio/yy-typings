macro_rules! create_guarded_uuid {
    ($this_val:ident) => {
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
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            /// Creates a new Id with the provided Uuid.
            pub fn with_id(id: uuid::Uuid) -> Self {
                Self(id)
            }

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

    pub mod sprite {
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

    pub mod texture_group;
    mod yyp;
    pub use yyp::*;
}
pub use typings::*;

pub mod utils {
    mod trailing_comma_utility;
    pub use trailing_comma_utility::TrailingCommaUtility;

    mod path_validator;
    pub use path_validator::PathValidator;
}
