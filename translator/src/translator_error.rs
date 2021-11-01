#[derive(Debug)]
pub enum TranslatorError {
    UrlParse {
        message: String,
    },
    Api {
        status_code: Option<reqwest::StatusCode>,
        message: String,
    },
}

impl std::fmt::Display for TranslatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UrlParse { message } => writeln!(f, "{}", message),
            Self::Api {
                status_code: Some(status_code),
                message,
            } => writeln!(
                f,
                "Status code {}, message {}",
                status_code.as_str(),
                message
            ),
            Self::Api { message, .. } => writeln!(f, "Message {}", message),
        }
    }
}

impl std::error::Error for TranslatorError {}

impl From<url::ParseError> for TranslatorError {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParse {
            message: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for TranslatorError {
    fn from(err: reqwest::Error) -> Self {
        Self::Api {
            status_code: err.status(),
            message: err.to_string(),
        }
    }
}
