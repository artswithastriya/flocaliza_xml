use flocariza_macros::localizable;
use serde_derive::{Deserialize, Serialize};
#[localizable]
#[derive(Deserialize, Serialize)]
struct App {
    pub name: Localized<String>,
    pub description: Localized<MarkupText>,
}

#[derive(Deserialize, Serialize)]
pub struct MarkupText {
    pub string: String,
}
fn main() {}
