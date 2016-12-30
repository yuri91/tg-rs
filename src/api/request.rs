use super::super::{serde_json,serde};
use std::default::Default;
use std::convert::Into;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Empty {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Update {
    pub offset: u64,
    pub timeout: i32
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Chat {
    pub chat_id: u64
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ParseMode {
    Markdown,
    HTML,
    Text
}
impl ParseMode {
    pub fn is_text(&self) -> bool {
        *self == ParseMode::Text
    }
}
impl Default for ParseMode {
    fn default() -> Self {
        ParseMode::Text
    }
}
#[derive(Clone, Debug, Serialize, Default)]
pub struct Message {
    pub chat_id: u64,
    pub text: String,
    pub disable_web_preview: bool,
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "ParseMode::is_text")]
    pub parse_mode: ParseMode,
    pub reply_to_message_id: Option<u64>,
    // reply_markup: ?
}
impl Message {
    pub fn new<S: Into<String>>(chat_id: u64, text: S) -> Message {
        Message {
            chat_id: chat_id,
            text: text.into(),
            ..Default::default()
        }
    }
    pub fn with_reply_to_message_id<T: Into<Option<u64>>>(mut self, rid: T) -> Self {
        self.reply_to_message_id = rid.into();
        self
    }
    pub fn wit_parse_mode(mut self, mode: ParseMode) -> Self {
        self.parse_mode = mode;
        self
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct InlineQueryAnswer<'a> {
    pub inline_query_id: &'a str,
    pub results: Vec<InlineQueryResult<'a>>,
    //..others
}
impl<'a> InlineQueryAnswer<'a> {
    pub fn new(id: &'a str, results: Vec<InlineQueryResult<'a>>) -> Self {
        InlineQueryAnswer::<'a> {
            inline_query_id: id,
            results: results,
        }
    }
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
        parse_mode: ParseMode
    },
    //..others
}
impl<'a> InputMessageContent<'a> {
    pub fn text(t: &'a str) -> Self {
        InputMessageContent::Text::<'a> {
            message_text: t,
            parse_mode: ParseMode::Text
        }
    }
}
impl <'a> serde::Serialize for InputMessageContent<'a> {
    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error>
        where S: ::serde::Serializer
    {
        match *self {
            InputMessageContent::Text {
                message_text,
                parse_mode
            }=> {
                let mut map = serde_json::Map::new();
                map.insert("message_text".to_string(),serde_json::Value::String(message_text.to_string()));
                if parse_mode != ParseMode::Text {
                    map.insert("parse_mode".to_string(),serde_json::to_value(&parse_mode));
                }
                serde_json::Value::Object(map).serialize(ser)
            },
        }
    }
}
