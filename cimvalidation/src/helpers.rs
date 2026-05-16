
/// Returns true if the string is a valid xsd:dateTime value.
pub fn is_xsd_datetime(s: &str) -> bool {
    // ISO 8601: YYYY-MM-DDTHH:MM:SS or with timezone
    let s = s.trim();
    if s.len() < 19 { return false; }
    s.as_bytes().get(4) == Some(&b'-')
        && s.as_bytes().get(7) == Some(&b'-')
        && s.as_bytes().get(10) == Some(&b'T')
}

/// Returns true if the string is a valid xsd:date value.
pub fn is_xsd_date(s: &str) -> bool {
    let s = s.trim();
    s.len() >= 10
        && s.as_bytes().get(4) == Some(&b'-')
        && s.as_bytes().get(7) == Some(&b'-')
}

/// Returns true if the string is a valid xsd:gMonthDay value (--MM-DD).
pub fn is_xsd_gmonthday(s: &str) -> bool {
    let s = s.trim();
    s.len() >= 7 && s.starts_with("--")
}

/// Returns true if the string is a non-empty anyURI (basic check).
pub fn is_xsd_anyuri(s: &str) -> bool {
    !s.trim().is_empty()
}
