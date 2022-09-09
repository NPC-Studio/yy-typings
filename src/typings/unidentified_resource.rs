use crate::{ResourceVersion, ViewPath};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

macro_rules! unidentified_resource {
    ($this_val:ident) => {
        /// This is a bodge to handle the fact that we don't currently have support for
        /// many of the Gms2 yy-files. Eventually, we'd like to support all of them, but
        /// downstream crates need to have some basic support until then. For now, this
        /// can be used for all top level files, providing the simplest of support.
        #[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct $this_val {
            /// The parent in the Gms2 virtual file system, ie. the parent which
            /// a user would see in the Navigation Pane in Gms2. This has no
            /// relationship to the actual operating system's filesystem.
            pub parent: ViewPath,

            /// The resource version of this yy file. At default 1.0.
            pub resource_version: ResourceVersion,

            /// The name of the object. This is the human readable name used in the IDE.
            pub name: String,
        }
    };
}

unidentified_resource!(AnimationCurve);
unidentified_resource!(Room);
unidentified_resource!(Extension);
unidentified_resource!(Font);
unidentified_resource!(Path);
unidentified_resource!(Sequence);
unidentified_resource!(Timeline);
