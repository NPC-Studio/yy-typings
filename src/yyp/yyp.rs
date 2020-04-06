use super::{parent_project::ParentProject, yyp_resource::YypResource};
use serde::{Deserialize, Serialize};

create_guarded_uuid!(ProjectId);

/// GMS2 project file typings
#[derive(Debug, Serialize, Deserialize)]
pub struct Yyp {
    /// Unknown property, seems to always be an empty array
    pub configs: Vec<String>,
    /// Contains project GUID
    pub id: ProjectId,
    /// Denotes whether this project uses drag and drop or not
    #[serde(rename = "IsDnDProject")]
    pub is_dnd_project: bool,

    #[serde(rename = "modelName")]
    pub model_name: ConstGmProject,

    /// A version number string, unknown use
    pub mvc: String,
    /// Allows for experimental JS editing. Unfinished or legacy feature. It's a secret.
    pub option_ecma: bool,
    /// Parent project, apparently non-public feature
    #[serde(rename = "parentProject")]
    pub parent_project: ParentProject,
    /// Contains all project resources (unordered)
    pub resources: Vec<YypResource>,

    /// An array of script GUID's
    pub script_order: Vec<String>,

    /// Unknown property, usually an empty string, unless you're making a tutorial
    /// in which case, shame upon your house
    pub tutorial: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConstGmProject {
    #[serde(rename = "GMProject")]
    GmProject,
}