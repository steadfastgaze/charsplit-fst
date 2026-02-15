// SPDX-License-Identifier: MIT OR Apache-2.0

//! Fugen-S detection and removal for German compound words.
//!
//! Fugen-S is a linking element "s" that appears in some German compounds,
//! often preceded by certain consonant combinations (ts, gs, ks, hls, ns).
//! When detecting compound boundaries, these endings are often removed
//! to find better probability matches.

/// Check if a word ends with a Fugen-S pattern that should be removed.
///
/// Returns `true` if the word ends with one of: ts, gs, ks, hls, ns
///
/// # Examples
///
/// ```
/// use charsplit_fst::fugen_s::should_remove_fugen_s;
///
/// assert!(should_remove_fugen_s("beits"));
/// assert!(should_remove_fugen_s("mehls"));
/// assert!(!should_remove_fugen_s("haus"));
/// ```
pub fn should_remove_fugen_s(s: &str) -> bool {
    s.ends_with("ts") || s.ends_with("gs") || s.ends_with("ks") || s.ends_with("hls") || s.ends_with("ns")
}

/// Remove Fugen-S from a word if applicable.
///
/// Returns `Some(&str)` with the last character removed if:
/// - The word ends with a Fugen-S pattern (ts, gs, ks, hls, ns)
/// - The result would be more than 2 characters long
///
/// Otherwise returns `None`.
///
/// # Examples
///
/// ```
/// use charsplit_fst::fugen_s::remove_fugen_s;
///
/// assert_eq!(remove_fugen_s("beits"), Some("beit"));
/// assert_eq!(remove_fugen_s("mehls"), Some("mehl"));
/// assert_eq!(remove_fugen_s("haus"), None);
/// assert_eq!(remove_fugen_s("ts"), None);  // Too short after removal
/// ```
pub fn remove_fugen_s(s: &str) -> Option<&str> {
    if should_remove_fugen_s(s) && s.len() > 3 {
        Some(&s[..s.len() - 1])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_remove_fugen_s_ts() {
        assert!(should_remove_fugen_s("beits"));
        assert!(should_remove_fugen_s("hilts"));
    }

    #[test]
    fn test_should_remove_fugen_s_gs() {
        assert!(should_remove_fugen_s("tags"));
        assert!(should_remove_fugen_s("lags"));
    }

    #[test]
    fn test_should_remove_fugen_s_ks() {
        assert!(should_remove_fugen_s("werks"));
        assert!(should_remove_fugen_s("riks"));
    }

    #[test]
    fn test_should_remove_fugen_s_hls() {
        assert!(should_remove_fugen_s("mehls"));
        assert!(should_remove_fugen_s("fahrls") == false);  // ends with "rls", not "hls"
        assert!(should_remove_fugen_s("wehrhls"));  // ends with "hls"
    }

    #[test]
    fn test_should_remove_fugen_s_ns() {
        assert!(should_remove_fugen_s("laufens"));  // ends with "ns"
        assert!(should_remove_fugen_s("teams") == false);  // ends with "ms", not "ns"
        assert!(should_remove_fugen_s("laufens"));  // correct example
    }

    #[test]
    fn test_should_not_remove_fugen_s() {
        assert!(!should_remove_fugen_s("haus"));
        assert!(!should_remove_fugen_s("baum"));
        assert!(!should_remove_fugen_s("s"));
        assert!(!should_remove_fugen_s("t"));  // Too short for any pattern
    }

    #[test]
    fn test_remove_fugen_s_ts() {
        assert_eq!(remove_fugen_s("beits"), Some("beit"));
        assert_eq!(remove_fugen_s("hilts"), Some("hilt"));
    }

    #[test]
    fn test_remove_fugen_s_gs() {
        assert_eq!(remove_fugen_s("tags"), Some("tag"));
    }

    #[test]
    fn test_remove_fugen_s_hls() {
        assert_eq!(remove_fugen_s("mehls"), Some("mehl"));
    }

    #[test]
    fn test_remove_fugen_s_no_match() {
        assert_eq!(remove_fugen_s("haus"), None);
        assert_eq!(remove_fugen_s("baum"), None);
    }

    #[test]
    fn test_remove_fugen_s_minimum_length() {
        // Don't remove if result would be <= 2 chars
        assert_eq!(remove_fugen_s("ats"), None);  // Would be "at" (2 chars, not > 2)
        assert_eq!(remove_fugen_s("ags"), None);  // Would be "ag" (2 chars)
        assert_eq!(remove_fugen_s("aks"), None);  // Would be "ak" (2 chars)
        assert_eq!(remove_fugen_s("ahls"), Some("ahl"));  // Would be "ahl" (3 chars, 3 > 2 is true)
        // "abcts" ends with "ts", 4 > 3, so should remove
        assert_eq!(remove_fugen_s("abcts"), Some("abct"));
    }

    #[test]
    fn test_remove_fugen_s_exactly_3_chars() {
        // 3 chars with ending pattern, but removal leaves 2 chars (not > 2)
        assert_eq!(remove_fugen_s("ats"), None);
        assert_eq!(remove_fugen_s("ags"), None);
    }

    #[test]
    fn test_remove_fugen_s_exactly_4_chars() {
        // 4 chars, removal leaves 3 chars (> 2)
        assert_eq!(remove_fugen_s("beits"), Some("beit"));  // "ts" removed
    }
}
