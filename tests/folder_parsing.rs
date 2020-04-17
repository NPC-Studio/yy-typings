use anyhow::Result;
use pretty_assertions::assert_eq;
use uuid::Uuid;
use yy_boss::{boss::GmFolderExt, yy_typings::resources::folder::*};

#[test]
fn trivial_case() -> Result<()> {
    let folder_example: &str = include_str!("./examples/gm_folder_example.yy");

    fn quick_id_maker(id: &str) -> GmFolderId {
        GmFolderId::with_id(Uuid::parse_str(id).unwrap())
    }

    let parse: GmFolder = serde_json::from_str(folder_example)?;
    let id = quick_id_maker("8ffab7ea-0bee-4ae6-be7b-a92447f0944a");

    let mockup_parse = GmFolder {
        id,
        model_name: ConstGmFolder::GmFolder,
        mvc: GmFolder::MVC.to_string(),
        name: id,
        children: vec![
            quick_id_maker("37b40014-5c69-4b59-bc54-c93d14f6e7a6"),
            quick_id_maker("c6293a12-a386-49e9-824f-66ba4d2edcae"),
            quick_id_maker("9b289e23-d09f-4b0c-bbb2-799496b1a538"),
            quick_id_maker("8350c2e6-3586-4d57-a961-b38f2c9ce45f"),
            quick_id_maker("2bf10e09-1ae0-4b2f-8365-03d777f66371"),
            quick_id_maker("9950c753-7465-4c08-a4fa-b52066d35fdc"),
            quick_id_maker("dc9c496c-efdc-49cd-b1dc-e784435e221b"),
            quick_id_maker("fa34a1d0-cda5-49ef-99db-45c7ac600537"),
            quick_id_maker("78a0e88a-2a9b-4597-897e-4172b8124768"),
            quick_id_maker("3b748d38-c841-4869-aba7-51791af9d508"),
            quick_id_maker("ddafd30e-761d-44f9-a5e1-728661efb8da"),
            quick_id_maker("c1d7b5b4-f4ac-48c4-bc40-f082d6227ffc"),
            quick_id_maker("1095d014-b52e-4029-952b-93bc709345ed"),
            quick_id_maker("8129d9e3-55e7-4520-a34d-0ffbbfc8621d"),
            quick_id_maker("a7584bba-5e70-41eb-b08b-ae1a88a97dfc"),
        ],
        filter_type: "root".to_string(),
        folder_name: "Default".to_string(),
        is_default_view: true,
        localised_folder_name: LocalisedFolderName::Empty,
    };

    assert_eq!(parse, mockup_parse);
    Ok(())
}

#[test]
fn builder() -> Result<()> {
    let folder_example: &str = include_str!("./examples/gm_folder_example.yy");

    fn quick_id_maker(id: &str) -> GmFolderId {
        GmFolderId::with_id(Uuid::parse_str(id).unwrap())
    }

    let parse: GmFolder = serde_json::from_str(folder_example)?;

    let mockup_parse = GmFolder::new_with_id(
        "Default".to_string(),
        quick_id_maker("8ffab7ea-0bee-4ae6-be7b-a92447f0944a"),
    )
    .root_folder()
    .children(&vec![
        quick_id_maker("37b40014-5c69-4b59-bc54-c93d14f6e7a6"),
        quick_id_maker("c6293a12-a386-49e9-824f-66ba4d2edcae"),
        quick_id_maker("9b289e23-d09f-4b0c-bbb2-799496b1a538"),
        quick_id_maker("8350c2e6-3586-4d57-a961-b38f2c9ce45f"),
        quick_id_maker("2bf10e09-1ae0-4b2f-8365-03d777f66371"),
        quick_id_maker("9950c753-7465-4c08-a4fa-b52066d35fdc"),
        quick_id_maker("dc9c496c-efdc-49cd-b1dc-e784435e221b"),
        quick_id_maker("fa34a1d0-cda5-49ef-99db-45c7ac600537"),
        quick_id_maker("78a0e88a-2a9b-4597-897e-4172b8124768"),
        quick_id_maker("3b748d38-c841-4869-aba7-51791af9d508"),
        quick_id_maker("ddafd30e-761d-44f9-a5e1-728661efb8da"),
        quick_id_maker("c1d7b5b4-f4ac-48c4-bc40-f082d6227ffc"),
        quick_id_maker("1095d014-b52e-4029-952b-93bc709345ed"),
        quick_id_maker("8129d9e3-55e7-4520-a34d-0ffbbfc8621d"),
        quick_id_maker("a7584bba-5e70-41eb-b08b-ae1a88a97dfc"),
    ]);

    assert_eq!(parse, mockup_parse);
    Ok(())
}
