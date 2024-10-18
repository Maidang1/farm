use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct HtmlConfig {
  pub base: Option<String>,
  pub disable_inline_scripts: bool,
}
