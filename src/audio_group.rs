use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct AudioGroup {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::AudioGroup, String>,
    #[default(-1)]
    pub targets: isize,
}

gm_const!(AudioGroup -> "GMAudioGroup");

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"{"resourceType":"GMAudioGroup","resourceVersion":"1.3","name":"audiogroup_default","targets":-1}"#;

    #[test]
    fn cycle() {
        let input: AudioGroup = serde_json::from_str(INPUT).unwrap();
        let output = serde_json::to_string(&input).unwrap();
        assert_eq!(INPUT, output);
    }
}
