use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "command", content = "args")]
pub enum CommandType {
    StartHTMLTag(String),
    AddAttribute(String, String),
    AddHTMLText(String),
    EndHTMLTag(String),
}
