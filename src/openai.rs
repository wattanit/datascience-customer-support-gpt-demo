use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message{
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub thread_id: String,
    pub role: String,
    pub content: Vec<MessageContent>,
    pub assistant_id: Option<String>,
    pub run_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContent{
    // renamed field type to typeString
    #[serde(rename = "type")]
    pub type_string: String,
    pub text: MessageContentText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContentText{
    pub value: String,
    pub annotations: Vec<MessageContentTextAnnotation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContentTextAnnotation{
    #[serde(rename = "type")]
    pub type_string: String,
    pub text: String,
    pub start_index: i32,
    pub end_index: i32,
    pub file_citation: Option<MessageContentTextAnnotationFileCitation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContentTextAnnotationFileCitation{
    pub file_id: String,
    pub quote: String,
}
