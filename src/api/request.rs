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
