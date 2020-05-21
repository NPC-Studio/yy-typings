/// These are the Tags which can be assigned to nearly **anything**
/// in the GMS2 editor. Users can add any UTF-8 valid tags,
/// though it is unclear if they can reliably READ non-ASCII tags.
///
/// Nonethless, they will be written into the JSON of each file, so these
/// typings reflect that UTF-8 validity.
pub type Tags = Vec<String>;
