use serde_json::Value;

#[derive(Clone,Debug,Deserialize)]
pub struct Response {
    pub ok: bool,
    pub result: Option<Value>,
    pub error_code: Option<u16>,
    pub description: Option<String>
}

#[derive(Clone,Debug,Deserialize)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>
}

#[derive(Clone,Debug,Deserialize)]
pub enum ChatKind {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "supergroup")]
    Supergroup,
    #[serde(rename = "channel")]
    Channel
}

#[derive(Clone,Debug,Deserialize)]
pub struct Chat {
    pub id: u64,
    #[serde(rename = "type")]
    pub chat_type: ChatKind,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub all_members_are_administrators: Option<bool>
}

