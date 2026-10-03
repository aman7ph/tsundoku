use url::Url;

use crate::utils::error::AppError;

/// Accepts only well-formed http(s) links. Returns the trimmed original text.
pub fn http_url(raw: &str) -> Result<String, AppError> {
    let raw = raw.trim();

    match Url::parse(raw) {
        Ok(parsed) if matches!(parsed.scheme(), "http" | "https") => Ok(raw.to_string()),
        _ => Err(AppError::BadRequest(
            "url must be a valid http or https link".into(),
        )),
    }
}

/// A required name: trimmed and not empty.
pub fn required_name(raw: &str) -> Result<String, AppError> {
    let name = raw.trim();

    if name.is_empty() {
        Err(AppError::BadRequest("name cannot be empty".into()))
    } else {
        Ok(name.to_string())
    }
}

/// Trims optional text and turns empty text into None.
pub fn clean_optional(value: Option<String>) -> Option<String> {
    value
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}
