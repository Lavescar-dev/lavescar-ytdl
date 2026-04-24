use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum YtdlpError {
    #[error("video is geo-blocked")]
    GeoBlocked,
    #[error("authentication required (cookies or login)")]
    AuthRequired,
    #[error("video not found or removed")]
    NotFound,
    #[error("network error: {0}")]
    NetworkError(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("shell error: {0}")]
    Shell(String),
    #[error("{0}")]
    Unknown(String),
}

impl YtdlpError {
    pub fn from_stderr(stderr: &str) -> Self {
        let s = stderr.to_lowercase();
        if s.contains("geo") || s.contains("country") {
            Self::GeoBlocked
        } else if s.contains("sign in") || s.contains("confirm your age") || s.contains("login") {
            Self::AuthRequired
        } else if s.contains("not found") || s.contains("removed") || s.contains("unavailable") {
            Self::NotFound
        } else if s.contains("network") || s.contains("connection") || s.contains("resolve") {
            Self::NetworkError(stderr.lines().next().unwrap_or(stderr).to_string())
        } else {
            Self::Unknown(stderr.lines().next().unwrap_or(stderr).to_string())
        }
    }

    /// Stable machine-readable discriminator for the frontend.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::GeoBlocked => "geo_blocked",
            Self::AuthRequired => "auth_required",
            Self::NotFound => "not_found",
            Self::NetworkError(_) => "network",
            Self::Io(_) => "io",
            Self::Json(_) => "parse",
            Self::Shell(_) => "shell",
            Self::Unknown(_) => "unknown",
        }
    }
}

impl Serialize for YtdlpError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geo_blocked_detected() {
        let err = YtdlpError::from_stderr(
            "ERROR: The uploader has not made this video available in your country",
        );
        assert_eq!(err.kind(), "geo_blocked");
    }

    #[test]
    fn auth_required_detected() {
        let err = YtdlpError::from_stderr("ERROR: Sign in to confirm your age");
        assert_eq!(err.kind(), "auth_required");
    }

    #[test]
    fn not_found_detected() {
        let err = YtdlpError::from_stderr(
            "ERROR: Video unavailable. This video has been removed by the uploader",
        );
        assert_eq!(err.kind(), "not_found");
    }

    #[test]
    fn network_detected() {
        let err = YtdlpError::from_stderr("ERROR: Unable to resolve host name");
        assert_eq!(err.kind(), "network");
    }

    #[test]
    fn unknown_fallback() {
        let err = YtdlpError::from_stderr("ERROR: something very strange");
        assert_eq!(err.kind(), "unknown");
    }

    #[test]
    fn first_line_only() {
        let stderr = "ERROR: not found\nWARNING: extra noise\nmore";
        match YtdlpError::from_stderr(stderr) {
            YtdlpError::NotFound => {}
            _ => panic!("expected NotFound"),
        }
    }
}
