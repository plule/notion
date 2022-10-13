use crate::models::FileOrEmojiObject;
use crate::models::WriteProperties;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct UpdatePageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The archived status of the page.
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Properties to modify.
    pub properties: Option<WriteProperties>,
    /// New icon of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileOrEmojiObject>,
}
