#[derive(Clone,Debug,Deserialize)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
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

// TODO: complete this
#[derive(Clone,Debug,Deserialize)]
pub struct Message {
    pub message_id: u64,
    pub from: Option<User>,
    pub date: u64,
    pub chat: Chat,
    pub text: Option<String>
}

#[derive(Clone,Debug,Deserialize)]
pub struct Update {
    pub update_id: u64,
    pub message: Option<Message>
}
