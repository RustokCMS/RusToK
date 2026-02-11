/// Input validation for content module DTOs
///
/// Provides validation rules and custom validators for all content-related inputs.

use validator::{Validate, ValidationError};

/// Custom validator for body format
pub fn validate_body_format(format: &str) -> Result<(), ValidationError> {
    match format {
        "markdown" | "html" | "plain" | "json" => Ok(()),
        _ => Err(ValidationError::new("invalid_format")),
    }
}

/// Custom validator for kind
pub fn validate_kind(kind: &str) -> Result<(), ValidationError> {
    match kind {
        "post" | "page" | "article" | "custom" => Ok(()),
        _ => Err(ValidationError::new("invalid_kind")),
    }
}

/// Custom validator for locale format (e.g., "en", "en-US", "ru-RU")
pub fn validate_locale(locale: &str) -> Result<(), ValidationError> {
    if locale.len() < 2 || locale.len() > 10 {
        return Err(ValidationError::new("invalid_locale_length"));
    }

    // Basic check: should be letters and hyphens only
    if !locale
        .chars()
        .all(|c| c.is_ascii_alphabetic() || c == '-')
    {
        return Err(ValidationError::new("invalid_locale_format"));
    }

    Ok(())
}

/// Custom validator for position (should be non-negative)
pub fn validate_position(position: &i32) -> Result<(), ValidationError> {
    if *position < 0 {
        return Err(ValidationError::new("position_must_be_non_negative"));
    }
    if *position > 100000 {
        return Err(ValidationError::new("position_too_large"));
    }
    Ok(())
}

/// Custom validator for depth (should be reasonable)
pub fn validate_depth(depth: &i32) -> Result<(), ValidationError> {
    if *depth < 0 {
        return Err(ValidationError::new("depth_must_be_non_negative"));
    }
    if *depth > 100 {
        return Err(ValidationError::new("depth_too_large"));
    }
    Ok(())
}

/// Custom validator for reply count
pub fn validate_reply_count(count: &i32) -> Result<(), ValidationError> {
    if *count < 0 {
        return Err(ValidationError::new("reply_count_must_be_non_negative"));
    }
    Ok(())
}

/// Custom validator for slug format
pub fn validate_slug(slug: &str) -> Result<(), ValidationError> {
    // Slug should be lowercase alphanumeric with hyphens
    if slug.is_empty() {
        return Err(ValidationError::new("slug_empty"));
    }

    if slug.len() > 255 {
        return Err(ValidationError::new("slug_too_long"));
    }

    // Check if slug matches pattern: lowercase letters, numbers, hyphens
    if !slug
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(ValidationError::new("slug_invalid_characters"));
    }

    // Should not start or end with hyphen
    if slug.starts_with('-') || slug.ends_with('-') {
        return Err(ValidationError::new("slug_hyphen_boundary"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_body_format_valid() {
        assert!(validate_body_format("markdown").is_ok());
        assert!(validate_body_format("html").is_ok());
        assert!(validate_body_format("plain").is_ok());
        assert!(validate_body_format("json").is_ok());
    }

    #[test]
    fn test_validate_body_format_invalid() {
        assert!(validate_body_format("xml").is_err());
        assert!(validate_body_format("unknown").is_err());
        assert!(validate_body_format("").is_err());
    }

    #[test]
    fn test_validate_kind_valid() {
        assert!(validate_kind("post").is_ok());
        assert!(validate_kind("page").is_ok());
        assert!(validate_kind("article").is_ok());
        assert!(validate_kind("custom").is_ok());
    }

    #[test]
    fn test_validate_kind_invalid() {
        assert!(validate_kind("invalid").is_err());
        assert!(validate_kind("").is_err());
    }

    #[test]
    fn test_validate_locale_valid() {
        assert!(validate_locale("en").is_ok());
        assert!(validate_locale("en-US").is_ok());
        assert!(validate_locale("ru-RU").is_ok());
        assert!(validate_locale("zh-CN").is_ok());
    }

    #[test]
    fn test_validate_locale_invalid() {
        assert!(validate_locale("e").is_err()); // Too short
        assert!(validate_locale("toolonglocale").is_err()); // Too long
        assert!(validate_locale("en_US").is_err()); // Underscore not allowed
        assert!(validate_locale("en123").is_err()); // Numbers not in right format
    }

    #[test]
    fn test_validate_position_valid() {
        assert!(validate_position(&0).is_ok());
        assert!(validate_position(&100).is_ok());
        assert!(validate_position(&10000).is_ok());
    }

    #[test]
    fn test_validate_position_invalid() {
        assert!(validate_position(&-1).is_err());
        assert!(validate_position(&-100).is_err());
        assert!(validate_position(&100001).is_err());
    }

    #[test]
    fn test_validate_depth_valid() {
        assert!(validate_depth(&0).is_ok());
        assert!(validate_depth(&5).is_ok());
        assert!(validate_depth(&50).is_ok());
    }

    #[test]
    fn test_validate_depth_invalid() {
        assert!(validate_depth(&-1).is_err());
        assert!(validate_depth(&101).is_err());
    }

    #[test]
    fn test_validate_slug_valid() {
        assert!(validate_slug("my-post").is_ok());
        assert!(validate_slug("hello-world").is_ok());
        assert!(validate_slug("post-123").is_ok());
        assert!(validate_slug("a").is_ok());
    }

    #[test]
    fn test_validate_slug_invalid() {
        assert!(validate_slug("").is_err());
        assert!(validate_slug("My-Post").is_err()); // Uppercase
        assert!(validate_slug("my_post").is_err()); // Underscore
        assert!(validate_slug("-mypost").is_err()); // Starts with hyphen
        assert!(validate_slug("mypost-").is_err()); // Ends with hyphen
        assert!(validate_slug("my post").is_err()); // Space
        assert!(validate_slug(&"a".repeat(256)).is_err()); // Too long
    }
}
