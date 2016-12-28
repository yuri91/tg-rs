use super::super::{serde,serde_json};
use super::super::errors;
use super::super::errors::ResultExt;

#[derive(Clone,Debug,Deserialize)]
pub struct Response {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub error_code: Option<u16>,
    pub description: Option<String>
}
impl Response {
    pub fn into_result<D: serde::Deserialize>(self) -> errors::Result<D> {
        match self.ok {
            true => {
                let response = serde_json::from_value(self.result.unwrap())
                    .chain_err(|| "cannot parse response".to_string())?;
                Ok(response)
            },
            false => bail!(errors::ErrorKind::ApiError(self.description.unwrap()))
        }
    }
}

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
struct UpdateWrapper {
    update_id: u64,
    message: Option<Message>,
    inline_query: Option<InlineQuery>,
    //..others
}
#[derive(Clone,Debug,Deserialize)]
pub enum UpdateKind {
    Message(Message),
    InlineQuery(InlineQuery),
    None
}
#[derive(Clone,Debug)]
pub struct Update {
    pub update_id: u64,
    pub content: UpdateKind
}
impl serde::Deserialize for Update {
    fn deserialize<D>(des: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        let wrapper = UpdateWrapper::deserialize(des)?;
        let content = match wrapper {
            UpdateWrapper { message: Some(m), .. } => UpdateKind::Message(m),
            UpdateWrapper { inline_query: Some(i), .. } => UpdateKind::InlineQuery(i),
            _ => UpdateKind::None
        };
        Ok(Update {
            update_id: wrapper.update_id,
            content: content
        })
    }
}

#[derive(Clone,Debug,Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    //location
    pub query: String,
    pub offset: String
}
