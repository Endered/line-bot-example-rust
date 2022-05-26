use line_bot_sdk::{
    error::AppError,
    models::message::{text::TextMessage, MessageObject},
};

pub async fn index() -> Result<Option<Vec<MessageObject>>, AppError> {
    /* Ok(Some(vec![MessageObject::Text(TextMessage::new(
        "友達追加ありがとうございます！".to_string(),
    ))])) */
    Ok(Some(vec![TextMessage::builder()
        .text("友達追加ありがとうございます！")
        .build()
        .into()]))
}
