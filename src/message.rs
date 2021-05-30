use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "result")]
pub enum SendMessageResponse {
    #[serde(rename = "success")]
    Success { id: i64, msg: String },
    #[serde(rename = "error")]
    Error {
        code: String,
        msg: String,
        stream: Option<String>,
    },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum SendMessageRequest {
    #[serde(rename = "stream")]
    Stream {
        to: String,
        topic: String,
        content: String,
    },
    #[serde(rename = "private")]
    Private { to: String, content: String },
}

#[derive(Serialize, Debug)]
pub struct GetMessagesRequest {
    pub anchor: Option<Anchor>,
    pub num_before: i64,
    pub num_after: i64,
    #[serde(skip_serializing)]
    pub narrow: Option<Vec<Narrow>>,
}

impl GetMessagesRequest {
    pub fn new(num_before: i64, num_after: i64) -> Self {
        Self {
            anchor: Some(Anchor::Newest),
            num_before,
            num_after,
            narrow: None,
        }
    }
    pub fn anchor(&mut self, anchor: Anchor) -> &mut Self {
        self.anchor = Some(anchor);
        self
    }
    pub fn narrow(&mut self, narrow: Vec<Narrow>) -> &mut Self {
        self.narrow = Some(narrow);
        self
    }
}

#[derive(Debug)]
pub enum Anchor {
    Newest,
    Oldest,
    FirstUnread,
    MessageID(i64),
}

impl Serialize for Anchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Anchor::Newest => serializer.serialize_str("newest"),
            Anchor::Oldest => serializer.serialize_str("oldest"),
            Anchor::FirstUnread => serializer.serialize_str("first_unread"),
            Anchor::MessageID(i) => serializer.serialize_i64(*i),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Narrow {
    pub operand: String,
    pub operator: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMessagesResponse {
    msg: String,
    result: String,
    anchor: i64,
    found_newest: bool,
    found_oldest: Option<bool>,
    found_anchor: bool,
    history_limited: Option<bool>,
    messages: Vec<ReceivedMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceivedMessage {
    avatar_url: String,
    client: String,
    content: String,
    content_type: String,
    display_recipient: DisplayRecipient,
    id: i64,
    is_me_message: bool,
    reactions: Vec<Reaction>,
    recipient_id: i64,
    sender_email: String,
    sender_full_name: String,
    sender_id: i64,
    sender_realm_str: String,
    stream_id: Option<i64>,
    subject: String,
    topic_links: Vec<String>,
    submessages: Vec<String>,
    timestamp: i64,
    r#type: String,
    flags: Vec<String>,
    last_edit_timestamp: Option<i64>,
    match_content: Option<String>,
    match_subject: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DisplayRecipient {
    Stream(String),
    PrivateMessage(Vec<DisplayRecipientPrivateMessage>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayRecipientPrivateMessage {
    id: i64,
    email: String,
    full_name: String,
    is_mirror_dummy: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    emoji_code: String,
    emoji_name: String,
    reaction_type: String,
    user_id: i64,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "result")]
#[serde(rename_all = "camelCase")]
pub enum DeleteMessageResponse {
    Success,
    Error(DeleteMessageError),
}

#[derive(Deserialize, Debug)]
pub struct DeleteMessageError {
    code: String,
    msg: String,
}

#[derive(Serialize, Debug)]
pub struct EditMessageRequest {
    #[serde(skip_serializing)]
    pub(crate) message_id: i64,
    topic: Option<String>,
    propagate_mode: PropagateMode,
    send_notification_to_old_thread: bool,
    send_notification_to_new_thread: bool,
    content: Option<String>,
    stream_id: Option<i64>,
}

impl EditMessageRequest {
    pub fn new(message_id: i64) -> Self {
        Self {
            message_id,
            topic: None,
            propagate_mode: PropagateMode::ChangeOne,
            send_notification_to_new_thread: true,
            send_notification_to_old_thread: true,
            content: None,
            stream_id: None,
        }
    }
    pub fn topic(&mut self, topic: &str) -> &mut Self {
        self.topic = Some(topic.to_string());
        self
    }
    pub fn propagate_mode(&mut self, propagate_mode: PropagateMode) -> &mut Self {
        self.propagate_mode = propagate_mode;
        self
    }
    pub fn send_notification_to_old_thread(&mut self, is_send: bool) -> &mut Self {
        self.send_notification_to_old_thread = is_send;
        self
    }
    pub fn send_notification_to_new_thread(&mut self, is_send: bool) -> &mut Self {
        self.send_notification_to_new_thread = is_send;
        self
    }
    pub fn content(&mut self, content: &str) -> &mut Self {
        self.content = Some(content.to_string());
        self
    }
    pub fn stream_id(&mut self, stream_id: i64) -> &mut Self {
        self.stream_id = Some(stream_id);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct AddEmojiReactionRequest {
    #[serde(skip_serializing)]
    pub(crate) message_id: i64,
    emoji_name: String,
    emoji_code: Option<String>,
    reaction_type: Option<ReactionType>,
}

impl AddEmojiReactionRequest {
    pub fn new(message_id: i64, emoji_name: &str) -> Self {
        Self {
            message_id,
            emoji_name: emoji_name.to_string(),
            emoji_code: None,
            reaction_type: None,
        }
    }
    pub fn emoji_code(&mut self, emoji_code: &str) -> &mut Self {
        self.emoji_code = Some(emoji_code.to_string());
        self
    }
    pub fn reaction_type(&mut self, reaction_type: ReactionType) -> &mut Self {
        self.reaction_type = Some(reaction_type);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct RemoveEmojiReactionRequest {
    pub(crate) message_id: i64,
    emoji_name: Option<String>,
    emoji_code: Option<String>,
    reaction_type: Option<ReactionType>,
}

impl RemoveEmojiReactionRequest {
    pub fn new(message_id: i64) -> Self {
        Self {
            message_id,
            emoji_name: None,
            emoji_code: None,
            reaction_type: None,
        }
    }
    pub fn emoji_name(&mut self, emoji_name: &str) -> &mut Self {
        self.emoji_name = Some(emoji_name.to_string());
        self
    }
    pub fn emoji_code(&mut self, emoji_code: &str) -> &mut Self {
        self.emoji_code = Some(emoji_code.to_string());
        self
    }
    pub fn reaction_type(&mut self, reaction_type: ReactionType) -> &mut Self {
        self.reaction_type = Some(reaction_type);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum ReactionType {
    UnicodeEmoji,
    RealmEmoji,
    ZulipExtraEmoji,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PropagateMode {
    ChangeOne,
    ChangeAll,
    ChangeLater,
}
