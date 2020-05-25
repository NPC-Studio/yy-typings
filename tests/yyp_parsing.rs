use include_dir::{include_dir, Dir, DirEntry};
use maplit::hashset;
use pretty_assertions::assert_eq;
use std::path::Path;
use yy_boss::{
    utils::TrailingCommaUtility,
    yy_typings::{
        texture_group::TextureGroup, AudioGroup, FilesystemPath, Yyp, YypConfig, YypFolder,
        YypIncludedFile, YypMetaData, YypResource,
    },
};

#[test]
fn trivial_yyp_parse() {
    let all_yyps: Dir = include_dir!("tests/examples/yyp_examples");
    let tcu = TrailingCommaUtility::new();

    for sprite_file in all_yyps.find("**/*.yyp").unwrap() {
        match sprite_file {
            DirEntry::File(file) => {
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                let _: Yyp = serde_json::from_str(&our_str).unwrap();
            }
            _ => {}
        }
    }
}

#[test]
fn deep_compare() {
    let test0_yyp = TrailingCommaUtility::clear_trailing_comma_once(include_str!(
        "./examples/yyp_examples/yyp_test0.yyp"
    ));

    let test0_yyp: Yyp = serde_json::from_str(&test0_yyp).unwrap();

    let test_yyp: Yyp = Yyp {
        resources: hashset![
            YypResource {
                id: FilesystemPath {
                    name: "Test4".to_string(),
                    path: Path::new(&format!("objects/{0}/{0}.yy", "Test4")).to_owned(),
                },
                order: 0,
            },
            YypResource {
                id: FilesystemPath {
                    name: "Object2".to_string(),
                    path: Path::new(&format!("objects/{0}/{0}.yy", "Object2")).to_owned(),
                },
                order: 1,
            },
            YypResource {
                id: FilesystemPath {
                    name: "Room2".to_string(),
                    path: Path::new(&format!("rooms/{0}/{0}.yy", "Room2")).to_owned(),
                },
                order: 1,
            },
            YypResource {
                id: FilesystemPath {
                    name: "Room1".to_string(),
                    path: Path::new(&format!("rooms/{0}/{0}.yy", "Room1")).to_owned(),
                },
                order: 0,
            },
        ],
        options: hashset![
            FilesystemPath {
                name: "Amazon Fire".to_string(),
                path: Path::new(&format!("options/amazonfire/options_amazonfire.yy")).to_owned(),
            },
            FilesystemPath {
                name: "Android".to_string(),
                path: Path::new(&format!("options/android/options_android.yy")).to_owned(),
            },
            FilesystemPath {
                name: "HTML5".to_string(),
                path: Path::new(&format!("options/html5/options_html5.yy")).to_owned(),
            },
            FilesystemPath {
                name: "iOS".to_string(),
                path: Path::new(&format!("options/ios/options_ios.yy")).to_owned(),
            },
            FilesystemPath {
                name: "Linux".to_string(),
                path: Path::new(&format!("options/linux/options_linux.yy")).to_owned(),
            },
            FilesystemPath {
                name: "macOS".to_string(),
                path: Path::new(&format!("options/mac/options_mac.yy")).to_owned(),
            },
            FilesystemPath {
                name: "Main".to_string(),
                path: Path::new(&format!("options/main/options_main.yy")).to_owned(),
            },
            FilesystemPath {
                name: "PlayStation 4".to_string(),
                path: Path::new(&format!("options/ps4/options_ps4.yy")).to_owned(),
            },
            FilesystemPath {
                name: "Switch".to_string(),
                path: Path::new(&format!("options/switch/options_switch.yy")).to_owned(),
            },
            FilesystemPath {
                name: "tvOS".to_string(),
                path: Path::new(&format!("options/tvos/options_tvos.yy")).to_owned(),
            },
            FilesystemPath {
                name: "Windows".to_string(),
                path: Path::new(&format!("options/windows/options_windows.yy")).to_owned(),
            },
            FilesystemPath {
                name: "Windows UWP".to_string(),
                path: Path::new(&format!("options/windowsuap/options_windowsuap.yy")).to_owned(),
            },
            FilesystemPath {
                name: "Xbox One".to_string(),
                path: Path::new(&format!("options/xboxone/options_xboxone.yy")).to_owned(),
            },
        ],
        is_dn_d_project: false,
        is_ecma: false,
        tutorial_path: String::new(),
        configs: YypConfig {
            name: "Default".to_string(),
            children: vec![
                YypConfig {
                    name: "TestConfig".to_string(),
                    children: vec![
                        YypConfig {
                            name: "SubChild".to_string(),
                            children: vec![],
                        },
                        YypConfig {
                            name: "NewConfig1".to_string(),
                            children: vec![],
                        },
                    ],
                },
                YypConfig {
                    name: "TestConfig2".to_string(),
                    children: vec![],
                },
            ],
        },
        room_order: vec![
            FilesystemPath {
                name: "Room1".to_string(),
                path: Path::new(&format!("rooms/{0}/{0}.yy", "Room1")).to_owned(),
            },
            FilesystemPath {
                name: "Room2".to_string(),
                path: Path::new(&format!("rooms/{0}/{0}.yy", "Room2")).to_owned(),
            },
        ],
        folders: hashset![
            YypFolder {
                folder_path: Path::new("folders/Sprites.yy").to_owned(),
                name: "Sprites".to_owned(),
                order: 1,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Tile Sets.yy").to_owned(),
                name: "Tile Sets".to_owned(),
                order: 2,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Sounds.yy").to_owned(),
                name: "Sounds".to_owned(),
                order: 3,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Paths.yy").to_owned(),
                name: "Paths".to_owned(),
                order: 4,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Scripts.yy").to_owned(),
                name: "Scripts".to_owned(),
                order: 5,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Shaders.yy").to_owned(),
                name: "Shaders".to_owned(),
                order: 6,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Fonts.yy").to_owned(),
                name: "Fonts".to_owned(),
                order: 7,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Timelines.yy").to_owned(),
                name: "Timelines".to_owned(),
                order: 8,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Objects.yy").to_owned(),
                name: "Objects".to_owned(),
                order: 9,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Rooms.yy").to_owned(),
                name: "Rooms".to_owned(),
                order: 10,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Sequences.yy").to_owned(),
                name: "Sequences".to_owned(),
                order: 11,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Animation Curves.yy").to_owned(),
                name: "Animation Curves".to_owned(),
                order: 12,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Notes.yy").to_owned(),
                name: "Notes".to_owned(),
                order: 13,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/Extensions.yy").to_owned(),
                name: "Extensions".to_owned(),
                order: 14,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: Path::new("folders/group1.yy").to_owned(),
                name: "group1".to_owned(),
                order: 15,
                ..YypFolder::default()
            },
        ],
        audio_groups: vec![AudioGroup::default()],
        texture_groups: vec![TextureGroup::default()],
        included_files: vec![
            YypIncludedFile {
                name: ".DS_Store".to_owned(),
                ..YypIncludedFile::default()
            },
            YypIncludedFile {
                name: "main.rtf".to_owned(),
                ..YypIncludedFile::default()
            },
            YypIncludedFile {
                name: "subfolder.rtf".to_owned(),
                file_path: Path::new("datafiles/test").to_owned(),
                ..YypIncludedFile::default()
            },
            YypIncludedFile {
                name: ".DS_Store".to_owned(),
                file_path: Path::new("datafiles/test").to_owned(),
                ..YypIncludedFile::default()
            },
        ],
        meta_data: YypMetaData::default(),
        name: "test2".to_string(),
        ..Yyp::default()
    };

    assert_eq!(test0_yyp, test_yyp);
}
