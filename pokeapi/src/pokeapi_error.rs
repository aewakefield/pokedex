#[derive(Debug)]
pub enum PokeapiError {
    UrlParse {
        message: String,
    },
    Api {
        status_code: Option<reqwest::StatusCode>,
        message: String,
    },
}

impl std::fmt::Display for PokeapiError {
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

impl From<url::ParseError> for PokeapiError {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParse {
            message: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for PokeapiError {
    fn from(err: reqwest::Error) -> Self {
        Self::Api {
            status_code: err.status(),
            message: err.to_string(),
        }
    }
}
