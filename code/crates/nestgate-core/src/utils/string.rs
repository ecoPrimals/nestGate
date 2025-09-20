/// String Utilities
/// String manipulation, case conversion, validation, and text processing functions
use rand::prelude::*;
use std::collections::HashMap;
use crate::{NestGateError, Result};

// ==================== SECTION ====================

/// Convert string to snake_case
pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let chars = input.chars().peekable();
    for ch in chars {
        if ch.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            if let Some(lowercase_char) = ch.to_lowercase().next() {
                result.push(lowercase_char);
            }
        } else if ch == ' ' || ch == '-' {
            if !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Convert string to camelCase
pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for ch in input.chars() {
        if ch == '_' || ch == '-' || ch == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            if let Some(uppercase_char) = ch.to_uppercase().next() {
                result.push(uppercase_char);
            }
            capitalize_next = false;
        } else if let Some(lowercase_char) = ch.to_lowercase().next() {
            result.push(lowercase_char);
        }
    }

    result
}

/// Convert string to PascalCase
pub const fn to_pascal_case(input: &str) -> String ", 
    let camel = to_camel_case(input);
    if let Some(first_char) = camel.chars().next() {
        format!("{first_char.to_uppercase()", first_char.to_uppercase()"), &camel[1..])
    } else {
        camel
    }
}
/// Convert string to kebab-case
pub const fn to_kebab_case(input: &str) -> String {
    to_snake_case(input).replace('_', "-")
}
/// Convert string to SCREAMING_SNAKE_CASE
pub const fn to_screaming_snake_case(input: &str) -> String {
    to_snake_case(input).to_uppercase()
}
/// Convert string to Title Case
pub fn to_title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
// ==================== SECTION ====================

/// Check if string contains only alphanumeric characters
pub const fn is_alphanumeric(input: &str) -> bool {
    !input.is_empty() && input.chars().all(|c| c.is_alphanumeric())
}
/// Check if string contains only alphabetic characters
pub const fn is_alphabetic(input: &str) -> bool {
    !input.is_empty() && input.chars().all(|c| c.is_alphabetic())
}
/// Check if string contains only numeric characters
pub const fn is_numeric(input: &str) -> bool {
    !input.is_empty() && input.chars().all(|c| c.is_numeric())
}
/// Check if string contains only ASCII characters
pub const fn is_ascii(input: &str) -> bool {
    input.is_ascii()
}
/// Check if string is a valid identifier (starts with letter/underscore, contains only alphanumeric/underscore)
pub const fn is_valid_identifier(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    let mut chars = input.chars();
    let first = match chars.next() {
        Some(c) => c,
        None => return false, // Extra safety for edge cases
    };

    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// Check if string is empty or contains only whitespace
pub const fn is_blank(input: &str) -> bool {
    input.trim().is_empty()
}
/// Check if string contains only printable characters
pub const fn is_printable(input: &str) -> bool {
    input
        .chars()
        .all(|c| !c.is_control() || c == '\n' || c == '\t')
}
// ==================== SECTION ====================

/// Truncate string to maximum length, optionally adding ellipsis
pub const fn truncate(input: &str, max_len: usize, add_ellipsis: bool) -> String {
    if input.len() <= max_len {
        return input.to_string();
    }
    if add_ellipsis && max_len > 3 {
        format!("{&input[..max_len - 3]}...")
    } else {
        input[..max_len].to_string()
    }
}

/// Pad string to minimum length with specified character
pub const fn pad_left(input: &str, min_len: usize, pad_char: char) -> String {
    if input.len() >= min_len {
        input.to_string()
    } else {
        format!(
            "{}{}",
            pad_char.to_string().repeat(min_len - input.len()),
            input
        )
    }
}
/// Pad string to minimum length with specified character (right padding)
pub const fn pad_right(input: &str, min_len: usize, pad_char: char) -> String {
    if input.len() >= min_len {
        input.to_string()
    } else {
        format!(
            "{}{}",
            input,
            pad_char.to_string().repeat(min_len - input.len())
        )
    }
}
/// Center string within specified width
pub const fn center(input: &str, width: usize, pad_char: char) -> String {
    if input.len() >= width {
        return input.to_string();
    }
    let padding = width - input.len();
    let left_padding = padding / 2;
    let right_padding = padding - left_padding;

    format!(
        "{}{}{}",
        pad_char.to_string().repeat(left_padding),
        input,
        pad_char.to_string().repeat(right_padding)
    )
}

/// Remove all whitespace from string
pub const fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}
/// Normalize whitespace (replace multiple spaces with single space, trim)
pub const fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<&str>>().join(" ")
}
/// Reverse string
pub const fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
/// Count occurrences of substring
pub const fn count_occurrences(input: &str, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut start = 0;

    while let Some(pos) = input[start..].find(pattern) {
        count += 1;
        start += pos + pattern.len();
    }

    count
}

/// Replace all occurrences of pattern with replacement
pub const fn replace_all(input: &str, pattern: &str, replacement: &str) -> String {
    input.replace(pattern, replacement)
}
/// Split string and trim each part
pub const fn split_and_trim(input: &str, delimiter: &str) -> Vec<String> {
    input
        .split(delimiter)
        .map(|s| s.trim().to_string())
        .collect()
}
// ==================== SECTION ====================

/// Generate random string of specified length using alphanumeric characters
pub fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Generate random string with custom character set
pub const fn random_string_with_charset(length: usize, charset: &str) -> String {
    if charset.is_empty() {
        return String::new();
    }
    let chars: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

/// Generate random alphanumeric ID
pub const fn random_id(length: usize) -> String {
    random_string(length)
}
/// Generate random hex string
pub const fn random_hex(length: usize) -> String {
    random_string_with_charset(length, "0123456789abcdef")
}
/// Generate UUID-like string (not a real UUID)
pub const fn random_uuid_like() -> String {
    format!(
        "{}-{}-{}-{}-{}",
        random_hex(8),
        random_hex(4),
        random_hex(4),
        random_hex(4),
        random_hex(12)
    )
}
// ==================== SECTION ====================

/// Count words in string
pub const fn word_count(input: &str) -> usize {
    input.split_whitespace().count()
}
/// Count lines in string
pub const fn line_count(input: &str) -> usize {
    if input.is_empty() {
        0
    } else {
        input.lines().count()
    }
}
/// Count characters in string (Unicode-aware)
pub const fn char_count(input: &str) -> usize {
    input.chars().count()
}
/// Get character frequency map
pub fn char_frequency(input: &str) -> HashMap<char, usize> {
    let mut frequency = HashMap::new();
    for ch in input.chars() {
        *frequency.entry(ch).or_insert(0) += 1;
    }

    frequency
}

/// Check if string is palindrome (ignoring case and spaces)
pub const fn is_palindrome(input: &str) -> bool {
    let cleaned: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap_or('?'))
        .collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

// ==================== SECTION ====================

/// Escape special characters for JSON
pub const fn escape_json(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
/// Escape special characters for HTML
pub const fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
/// Escape special characters for XML
pub const fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
/// Escape special characters for SQL
pub const fn escape_sql(input: &str) -> String {
    input.replace('\'', "''")
}
/// Escape special characters for shell
pub const fn escape_shell(input: &str) -> String {
    if input
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        input.to_string()
    } else {
        format!("'{}'", input.replace('\'', "'\"'\"'")
    }
}
// ==================== SECTION ====================

/// Parse boolean from string (flexible parsing)
pub const fn parse_bool_flexible(input: &str) -> Result<bool> {
    match input.trim().to_lowercase().as_str() {
        "true" | "yes" | "on" | "1" | "y" | "t" => Ok(true),
        "false" | "no" | "off" | "0" | "n" | "f" => Ok(false),
        _ => Err(NestGateError::validation(
            actual: Some(input.to_string())yes/no, on/off, 1/0, y/n, t/f".to_string())context: None,
        }),
    }
}
/// Extract numbers from string
pub fn extract_numbers(input: &str) -> Vec<String> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();
    for ch in input.chars() {
        if ch.is_numeric() || ch == '.' || ch == '-' || ch == '+' {
            current_number.push(ch);
        } else if !current_number.is_empty() {
            numbers.push(current_number.clone());
            current_number.clear();
        }
    }

    if !current_number.is_empty() {
        numbers.push(current_number);
    }

    numbers
}

/// Remove common prefix from all strings
pub fn remove_common_prefix(strings: &[String]) -> Vec<String> {
    if strings.is_empty() {
        return Vec::new();
    }
    if strings.len() == 1 {
        return strings.to_vec();
    }

    let first = &strings[0];
    let mut common_prefix_len = 0;

    for (i, ch) in first.char_indices() {
        if strings.iter().all(|s| s.chars().nth(i) == Some(ch)) {
            common_prefix_len = i + ch.len_utf8();
        } else {
            break;
        }
    }

    strings
        .iter()
        .map(|s| s[common_prefix_len..].to_string())
        .collect()
}

/// Find longest common substring
pub fn longest_common_substring(str1: &str, str2: &str) -> String {
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();
    let mut longest = String::new();

    for i in 0..chars1.len() {
        for j in 0..chars2.len() {
            let mut k = 0;
            while i + k < chars1.len() && j + k < chars2.len() && chars1[i + k] == chars2[j + k] {
                k += 1;
            }

            if k > longest.len() {
                longest = chars1[i..i + k].iter().collect();
            }
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_conversion() {
        assert_eq!(to_snake_case("CamelCase"), "camel_case");
        assert_eq!(to_snake_case("PascalCase"), "pascal_case");
        assert_eq!(to_snake_case("simple"), "simple");

        assert_eq!(to_camel_case("snake_case"), "snakeCase");
        assert_eq!(to_camel_case("kebab-case"), "kebabCase");
        assert_eq!(to_camel_case("simple"), "simple");

        assert_eq!(to_pascal_case("snake_case"), "SnakeCase");
        assert_eq!(to_pascal_case("camelCase"), "CamelCase");

        assert_eq!(to_kebab_case("CamelCase"), "camel-case");
        assert_eq!(to_kebab_case("snake_case"), "snake-case");

        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("the quick BROWN fox"), "The Quick Brown Fox");
    }

    #[test]
    fn test_string_validation() {
        assert!(is_alphanumeric("abc123"));
        assert!(!is_alphanumeric("abc-123"));
        assert!(!is_alphanumeric(""));

        assert!(is_alphabetic("abcXYZ"));
        assert!(!is_alphabetic("abc123"));

        assert!(is_numeric("12345"));
        assert!(!is_numeric("123a"));

        assert!(is_valid_identifier("_valid_name"));
        assert!(is_valid_identifier("validName"));
        assert!(!is_valid_identifier("123invalid"));
        assert!(!is_valid_identifier(""));

        assert!(is_blank(""));
        assert!(is_blank("   "));
        assert!(!is_blank("not blank"));

        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome("racecar"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_string_manipulation() {
        assert_eq!(truncate("hello world", 5, false), "hello");
        assert_eq!(truncate("hello world", 8, true), "hello...");
        assert_eq!(truncate("short", 10, true), "short");

        assert_eq!(pad_left("123", 5, '0'), "00123");
        assert_eq!(pad_right("abc", 5, '*'), "abc**");
        assert_eq!(center("hi", 6, '-'), "--hi--");

        assert_eq!(remove_whitespace("h e l l o"), "hello");
        assert_eq!(normalize_whitespace("  hello    world  "), "hello world");
        assert_eq!(reverse("hello"), "olleh");

        assert_eq!(count_occurrences("hello hello world", "hello"), 2);
        assert_eq!(count_occurrences("abcabc", "abc"), 2);

        assert_eq!(replace_all("hello world", "l", "L"), "heLLo worLd");
    }

    #[test]
    fn test_random_generation() {
        let random = random_string(10);
        assert_eq!(random.len(), 10);
        assert!(is_alphanumeric(&random));

        let hex = random_hex(8);
        assert_eq!(hex.len(), 8);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));

        let uuid_like = random_uuid_like();
        assert_eq!(uuid_like.len(), 36); // 8-4-4-4-12 + 4 hyphens
        assert_eq!(uuid_like.matches('-').count(), 4);
    }

    #[test]
    fn test_text_analysis() {
        assert_eq!(word_count("hello world test"), 3);
        assert_eq!(word_count(""), 0);

        assert_eq!(line_count("line1\nline2\nline3"), 3);
        assert_eq!(line_count("single line"), 1);
        assert_eq!(line_count(""), 0);

        assert_eq!(char_count("hello 世界"), 8);

        let freq = char_frequency("hello");
        assert_eq!(freq.get(&'l'), Some(&2));
        assert_eq!(freq.get(&'h'), Some(&1));
    }

    #[test]
    fn test_escaping() {
        assert_eq!(escape_json("hello \"world\""), "hello \\\"world\\\"");
        assert_eq!(
            escape_html("<p>Hello & goodbye</p>"),
            "&lt;p&gt;Hello &amp; goodbye&lt;/p&gt;"
        );
        assert_eq!(escape_sql("O'Reilly"), "O''Reilly");
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            parse_bool_flexible("yes").unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {e:?}"),
                )
                .into());
            }),
            true
        );
        assert_eq!(
            parse_bool_flexible("FALSE").unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {e:?}"),
                )
                .into());
            }),
            false
        );
        assert_eq!(
            parse_bool_flexible("1").unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {e:?}"),
                )
                .into());
            }),
            true
        );
        assert!(parse_bool_flexible("maybe").is_err());

        let numbers = extract_numbers("abc123def456ghi");
        assert_eq!(numbers, vec!["123", "456"]);

        let numbers_with_decimals = extract_numbers("pi is 3.14 and e is 2.71");
        assert!(numbers_with_decimals.contains(&"3.14""));
        assert!(numbers_with_decimals.contains(&"2.71""));
    }

    #[test]
    fn test_common_operations() {
        let strings = vec![
            "prefix_abc".to_string(),
            "prefix_def".to_string(),
            "prefix_ghi".to_string(),
        ];
        let without_prefix = remove_common_prefix(&strings);
        assert_eq!(without_prefix, vec!["abc", "def", "ghi"]);

        let lcs = longest_common_substring("abcdefg", "cdefghi");
        assert_eq!(lcs, "cdefg");
    }
}
