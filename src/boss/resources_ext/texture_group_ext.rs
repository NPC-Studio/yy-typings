use super::yy_typings::texture_group::{GenerateMipMaps, TextureGroup, TextureGroupId};
use regex::Regex;
use serde_json::Value;

#[derive(Debug)]
pub struct TextureGroupController {
    default_group: TextureGroup,
    texture_groups: Vec<TextureGroup>,
}

impl TextureGroupController {
    pub fn default_group(&self) -> &TextureGroup {
        &self.default_group
    }

    pub fn texture_groups(&self) -> &[TextureGroup] {
        &self.texture_groups
    }
}

pub trait TextureGroupExt {
    const REGEX_OPTIONS: &'static str = r"←([0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12})\|(\{\r?\n[\s\S]+?\n\})";

    fn parse_options_file(text: &str) -> anyhow::Result<TextureGroupController>;
    fn new(target: usize, texture_group_id: TextureGroupId, texture_group_name: String) -> Self;
    fn autocrop(self, autocrop: bool) -> Self;
    fn mipmaps(self, mipmap: GenerateMipMaps) -> Self;
    fn border_size(self, border_size: usize) -> Self;
    fn group_parent(self, group_parent: Option<TextureGroupId>) -> Self;
    fn scaled(self, scaled: bool) -> Self;
}

impl TextureGroupExt for TextureGroup {
    fn parse_options_file(text: &str) -> anyhow::Result<TextureGroupController> {
        let re = Regex::new(Self::REGEX_OPTIONS)?;
        let default_texture_id = TextureGroupId::with_id(
            uuid::Uuid::parse_str("1225f6b0-ac20-43bd-a82e-be73fa0b6f4f").unwrap(),
        );

        let mut default_group = None;
        let mut texture_groups = vec![];

        for cap in re.captures_iter(text) {
            let mut accept_default_tex_group = false;

            for subcap in cap.iter() {
                if let Some(subcap) = subcap {
                    if accept_default_tex_group == false {
                        // Chaos to make it into a string...
                        let uuid_string = format!("\"{}\"", subcap.as_str());
                        if let Ok(value) = serde_json::from_str(&uuid_string) {
                            let as_tex: TextureGroupId = value;
                            accept_default_tex_group = as_tex == default_texture_id;
                            continue;
                        }
                    }

                    if let Ok(val) = serde_json::from_str(subcap.as_str()) {
                        let val: Value = val;
                        match serde_json::from_value(val.clone()) {
                            Ok(maybe_value) => {
                                if accept_default_tex_group {
                                    default_group = Some(maybe_value);
                                    continue;
                                }
                            }
                            Err(_) => {
                                if let Some(object) = val.as_object() {
                                    if let Some(object) = object.get("textureGroups") {
                                        if let Some(object) = object.get("Additions") {
                                            if let Some(tex_array) = object.as_array() {
                                                for member in tex_array.iter() {
                                                    let member: &Value = member;
                                                    if let Some(object) = member.get("Value") {
                                                        match serde_json::from_value(object.clone())
                                                        {
                                                            Ok(value) => {
                                                                texture_groups.push(value);
                                                            }
                                                            Err(_) => {}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    };
                }
            }
        }

        let default_group =
            default_group.ok_or_else(|| anyhow::anyhow!("No default texture group found!"))?;

        Ok(TextureGroupController {
            default_group,
            texture_groups,
        })
    }

    fn new(targets: usize, texture_group_id: TextureGroupId, texture_group_name: String) -> Self {
        Self {
            targets,
            id: texture_group_id,
            mvc: "1.0".to_string(),
            group_name: texture_group_name,
            model_name: Default::default(),
            autocrop: true,
            border: 2,
            group_parent: TextureGroupId::default(),
            mips_to_generate: Default::default(),
            scaled: true,
        }
    }

    fn autocrop(mut self, autocrop: bool) -> Self {
        self.autocrop = autocrop;
        self
    }

    fn group_parent(mut self, group_parent: Option<TextureGroupId>) -> Self {
        self.group_parent = group_parent.unwrap_or_default();
        self
    }

    fn scaled(mut self, scaled: bool) -> Self {
        self.scaled = scaled;
        self
    }

    fn mipmaps(mut self, mipmap: GenerateMipMaps) -> Self {
        self.mips_to_generate = mipmap;
        self
    }
    fn border_size(mut self, border_size: usize) -> Self {
        self.border = border_size;
        self
    }
}
