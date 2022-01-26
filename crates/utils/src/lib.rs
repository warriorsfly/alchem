// use std::{fmt, fmt::Display};
// use tracing_error::SpanTrace;

pub mod apub;
pub mod claims;
pub mod rate_limit;
pub mod settings;
pub mod utils;
/// local user id
pub type LocalUserId = usize;
/// websocket connection id
pub type ConnectionId = usize;
/// room id
pub type RoomId = usize;

// #[derive(PartialEq, Eq, Hash, Debug, Clone)]
// pub struct IpAddr(pub String);

// impl fmt::Display for IpAddr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// #[macro_export]
// macro_rules! location_info {
//     () => {
//         format!(
//             "None value at {}:{}, column {}",
//             file!(),
//             line!(),
//             column!()
//         )
//     };
// }

// #[derive(serde::Serialize)]
// struct DoZeApiError {
//     error: &'static str,
// }

// pub struct DoubleZeroError {
//     pub code: Option<i32>,
//     pub message: Option<&'static str>,
//     pub inner: anyhow::Error,
//     pub context: SpanTrace,
// }

// impl DoubleZeroError {
//     pub fn from_code(self, code: i32) -> Self {
//         Self {
//             code: Some(code),
//             ..self
//         }
//     }

//     pub fn from_message(self, message: &'static str) -> Self {
//         Self {
//             message: Some(message),
//             ..self
//         }
//     }

//     pub fn with_message(self, message: &'static str) -> Self {
//         Self {
//             message: Some(message),
//             ..self
//         }
//     }

//     pub fn to_json(&self) -> Result<String, Self> {
//         let api_error = match self.message {
//             Some(error) => DoZeApiError { error },
//             None => DoZeApiError { error: "Unknown" },
//         };

//         Ok(serde_json::to_string(&api_error)?)
//     }
// }

// impl<T> From<T> for DoubleZeroError
// where
//     T: Into<anyhow::Error>,
// {
//     fn from(t: T) -> Self {
//         Self {
//             code: None,
//             message: None,
//             inner: t.into(),
//             context: SpanTrace::capture(),
//         }
//     }
// }

// impl std::fmt::Debug for DoubleZeroError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("DoubleZeroError")
//             .field("code", &self.code)
//             .field("message", &self.message)
//             .field("inner", &self.inner)
//             .field("context", &self.context)
//             .finish()
//     }
// }

// impl Display for DoubleZeroError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if let Some(code) = self.code {
//             write!(f, "{}: ", code)?;
//         }
//         if let Some(message) = self.message {
//             write!(f, "{}: ", message)?;
//         }
//         writeln!(f, "{}", self.inner)?;
//         self.context.fmt(f)
//     }
// }

// impl actix_web::error::ResponseError for DoubleZeroError {
//     fn status_code(&self) -> StatusCode {
//         match self.inner.downcast_ref::<diesel::result::Error>() {
//             Some(diesel::result::Error::NotFound) => StatusCode::NOT_FOUND,
//             _ => StatusCode::BAD_REQUEST,
//         }
//     }

//     fn error_response(&self) -> HttpResponse {
//         if let Some(message) = &self.message {
//             HttpResponse::build(self.status_code()).json(DoZeApiError { error: message })
//         } else {
//             HttpResponse::build(self.status_code())
//                 .content_type("text/plain")
//                 .body(self.inner.to_string())
//         }
//     }
// }
