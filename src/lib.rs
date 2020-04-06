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

            /// Gives access to the inner ID. Try to not use this one too much!
            pub fn inner(&self) -> uuid::Uuid {
                self.0
            }
        }
    };
}

pub mod resources {
    use super::*;
    pub mod sprite;
}

pub mod yyp {
    mod parent_project;
    pub use parent_project::*;
    mod yyp;
    pub use yyp::*;
    mod yyp_resource;
    pub use yyp_resource::*;
}

mod yy_boss;
pub use yy_boss::YyBoss;
mod yy_resource;
pub use yy_resource::YyResource;

type YyResult<T> = Result<T, Box<dyn std::error::Error>>;
