use super::super::{serde_json,serde};

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Empty {
}
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Update {
    pub offset: u64,
    pub timeout: i32
}
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Chat {
    pub chat_id: u64
}
#[derive(Clone, Copy, Debug, Serialize)]
pub enum ParseMode {
    Markdown,
    HTML
}
#[derive(Clone, Debug, Serialize, Default)]
pub struct Message<'a> {
    pub chat_id: u64,
    pub text: &'a str,
    pub disable_web_preview: bool,
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    pub reply_to_message_id: Option<u64>,
    // reply_markup: ?
}

#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryAnswer<'a> {
    pub inline_query_id: &'a str,
    pub results: Vec<InlineQueryResult<'a>>,
    //..others
}

#[derive(Clone, Debug)]
pub enum InlineQueryResult<'a> {
    Article {
        id: &'a str,
        title: &'a str,
        input_message_content: InputMessageContent<'a>,
        //..others
    },
    //..others
}
impl <'a> serde::Serialize for InlineQueryResult<'a> {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error>
        where S: ::serde::Serializer
    {
        match *self {
            InlineQueryResult::Article {
                id,title,ref input_message_content
            }=> {
                let mut map = serde_json::Map::new();
                map.insert("type".to_string(),serde_json::Value::String("article".to_string()));
                map.insert("id".to_string(),serde_json::Value::String(id.to_string()));
                map.insert("title".to_string(),serde_json::Value::String(title.to_string()));
                map.insert("input_message_content".to_string(),serde_json::to_value(input_message_content));
                serde_json::Value::Object(map).serialize(ser)
            },
        }
    }
}
#[derive(Clone, Debug)]
pub enum InputMessageContent<'a> {
    Text {
        message_text: &'a str,
        //..others
    },
    //..others
}
impl <'a> serde::Serialize for InputMessageContent<'a> {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error>
        where S: ::serde::Serializer
    {
        match *self {
            InputMessageContent::Text {
                message_text
            }=> {
                let mut map = serde_json::Map::new();
                map.insert("message_text".to_string(),serde_json::Value::String(message_text.to_string()));
                serde_json::Value::Object(map).serialize(ser)
            },
        }
    }
}
