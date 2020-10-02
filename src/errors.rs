use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum NotificationError {
    #[error("Message could not be parsed. Details: {details}.")]
    MessageError { details: String },

    #[error("Unexpected response notification destiny. Details: {details}.")]
    ApiError { details: String },

    #[error(
        "Notificator take to mush processing the message, {time} milliseconds. Details: {details}."
    )]
    TimeoutError { time: u64, details: String },

    #[error("Notificator internal error. Details: {details}")]
    InternalError { details: String },
}
