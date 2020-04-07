use super::yyp::ProjectId;
use super::yyp_resource::YypResource;
use serde::{Deserialize, Serialize};

/// Parent project entry of a YYP
///
/// Parent project, apparently non-public feature
#[derive(Debug, Serialize, Deserialize)]
pub struct ParentProject {
    /// GUID of the parent project
    pub id: ProjectId,
    /// Describes object entry type.
    /// Always "GMParentProject"
    #[serde(rename = "modelName")]
    pub model_name: ConstGmProjectParent,
    /// A version number string, unknown use
    pub mvc: String,
    /// Contains parent project resources
    #[serde(rename = "alteredResources")]
    pub altered_resources: Vec<YypResource>,

    /// Unkown property, usually an empty array
    #[serde(rename = "hiddenResources")]
    pub hidden_resources: Vec<YypResource>,
    /// Contains parent project path representation. Not an actual system path.
    #[serde(rename = "projectPath")]
    pub project_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConstGmProjectParent {
    #[serde(rename = "GMProjectParent")]
    GmProjectParent,
}
