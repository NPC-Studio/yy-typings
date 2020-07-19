use include_dir::{include_dir, Dir, DirEntry};
use pretty_assertions::assert_eq;
use std::path::Path;
use yy_typings::{
    texture_group::TextureGroup, utils::TrailingCommaUtility, AudioGroup, FilesystemPath,
    ViewPathLocation, Yyp, YypConfig, YypFolder, YypIncludedFile, YypMetaData, YypResource,
};

#[test]
fn trivial_yyp_parse() {
    let all_yyps: Dir = include_dir!("tests/examples/yyp_examples");
    let tcu = TrailingCommaUtility::new();

    for sprite_file in all_yyps.find("**/*.yyp").unwrap() {
        if let DirEntry::File(file) = sprite_file {
            let our_str = std::str::from_utf8(file.contents()).unwrap();
            let our_str = tcu.clear_trailing_comma(our_str);
            let _: Yyp = serde_json::from_str(&our_str).unwrap();
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
        resources: vec![
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
        options: vec![
            FilesystemPath {
                name: "Amazon Fire".to_string(),
                path: Path::new("options/amazonfire/options_amazonfire.yy").to_owned(),
            },
            FilesystemPath {
                name: "Android".to_string(),
                path: Path::new("options/android/options_android.yy").to_owned(),
            },
            FilesystemPath {
                name: "HTML5".to_string(),
                path: Path::new("options/html5/options_html5.yy").to_owned(),
            },
            FilesystemPath {
                name: "iOS".to_string(),
                path: Path::new("options/ios/options_ios.yy").to_owned(),
            },
            FilesystemPath {
                name: "Linux".to_string(),
                path: Path::new("options/linux/options_linux.yy").to_owned(),
            },
            FilesystemPath {
                name: "macOS".to_string(),
                path: Path::new("options/mac/options_mac.yy").to_owned(),
            },
            FilesystemPath {
                name: "Main".to_string(),
                path: Path::new("options/main/options_main.yy").to_owned(),
            },
            FilesystemPath {
                name: "PlayStation 4".to_string(),
                path: Path::new("options/ps4/options_ps4.yy").to_owned(),
            },
            FilesystemPath {
                name: "Switch".to_string(),
                path: Path::new("options/switch/options_switch.yy").to_owned(),
            },
            FilesystemPath {
                name: "tvOS".to_string(),
                path: Path::new("options/tvos/options_tvos.yy").to_owned(),
            },
            FilesystemPath {
                name: "Windows".to_string(),
                path: Path::new("options/windows/options_windows.yy").to_owned(),
            },
            FilesystemPath {
                name: "Windows UWP".to_string(),
                path: Path::new("options/windowsuap/options_windowsuap.yy").to_owned(),
            },
            FilesystemPath {
                name: "Xbox One".to_string(),
                path: Path::new("options/xboxone/options_xboxone.yy").to_owned(),
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
        folders: vec![
            YypFolder {
                folder_path: ViewPathLocation("folders/Sprites.yy".to_string()),
                name: "Sprites".to_owned(),
                order: 1,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Tile Sets.yy".to_string()),
                name: "Tile Sets".to_owned(),
                order: 2,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Sounds.yy".to_string()),
                name: "Sounds".to_owned(),
                order: 3,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Paths.yy".to_string()),
                name: "Paths".to_owned(),
                order: 4,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Scripts.yy".to_string()),
                name: "Scripts".to_owned(),
                order: 5,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Shaders.yy".to_string()),
                name: "Shaders".to_owned(),
                order: 6,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Fonts.yy".to_string()),
                name: "Fonts".to_owned(),
                order: 7,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Timelines.yy".to_string()),
                name: "Timelines".to_owned(),
                order: 8,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Objects.yy".to_string()),
                name: "Objects".to_owned(),
                order: 9,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Rooms.yy".to_string()),
                name: "Rooms".to_owned(),
                order: 10,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Sequences.yy".to_string()),
                name: "Sequences".to_owned(),
                order: 11,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Animation Curves.yy".to_string()),
                name: "Animation Curves".to_owned(),
                order: 12,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Notes.yy".to_string()),
                name: "Notes".to_owned(),
                order: 13,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/Extensions.yy".to_string()),
                name: "Extensions".to_owned(),
                order: 14,
                ..YypFolder::default()
            },
            YypFolder {
                folder_path: ViewPathLocation("folders/group1.yy".to_string()),
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
