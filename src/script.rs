use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::VersionStamp;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::Script>,

    // /// The version that GM uses -- it's currently "v1"
    #[serde(rename = "$GMScript")]
    pub gm_version_stamp: VersionStamp<1>,

    /// Is this an autogenerated compatibility script?
    pub is_compatibility: bool,

    /// Is this used in DragNDrop? Hopefully not! that would get messy.
    pub is_dn_d: bool,

    pub parent: crate::ViewPath,
}

gm_const!(Script -> "GMScript");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utils::TrailingCommaUtility, ResourceVersion, ViewPath, ViewPathLocation};
    use include_dir::{include_dir, Dir, DirEntry};
    use pretty_assertions::assert_eq;

    static ALL_SCRIPTS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/data/scripts");

    #[test]
    fn trivial_sprite_parsing() {
        let tcu = TrailingCommaUtility::new();

        for object_file in ALL_SCRIPTS.find("**/*.yy").unwrap() {
            if let DirEntry::File(file) = object_file {
                println!("parsing {}", file.path().display());
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                serde_json::from_str::<Script>(&our_str).unwrap();
            }
        }
    }

    #[test]
    fn deep_equality() {
        let script_raw = include_str!("../data/scripts/Camera.yy");

        let script_parsed: Script =
            serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(script_raw))
                .unwrap();

        let script = Script {
            common_data: crate::CommonData {
                resource_version: ResourceVersion::default(),
                name: "Camera".to_string(),
                resource_type: consts::Script,
            },
            gm_version_stamp: VersionStamp,
            is_dn_d: false,
            is_compatibility: false,
            parent: ViewPath {
                name: "Camera".to_string(),
                path: ViewPathLocation("folders/Scripts/Gameplay Systems/Camera.yy".to_string()),
            },
        };

        assert_eq!(script_parsed, script);
    }
}
