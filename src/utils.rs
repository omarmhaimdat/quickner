use std::collections::HashSet;

use indicatif::{ProgressBar, ProgressStyle};
use log::debug;

use crate::config::Filters;

/// Checks if a string is alphanumeric.
/// # Examples
/// ```
/// use utils::is_alphanumeric;
/// let text = "Hello, world!";
/// assert_eq!(is_alphanumeric(text), true);
/// ```
pub(crate) fn is_alphanumeric(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    text.chars().all(|c| c.is_alphanumeric())
}

/// Checks if a string contains punctuation.
/// # Examples
/// ```
/// use utils::contains_punctuation;
/// let text = "Hello, world!";
/// assert_eq!(contains_punctuation(text), true);
/// ```
pub(crate) fn contains_punctuation(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    text.chars().any(|c| c.is_ascii_punctuation())
}

/// Checks if a string contains numbers.
/// # Examples
/// ```
/// use utils::contains_numbers;
/// let text_without = "Hello, world!";
/// assert_eq!(contains_numbers(text), false);
/// let text_with = "Hello, 123!";
/// assert_eq!(contains_numbers(text), true);
/// ```
/// # Panics
/// Panics if the string contains non-ASCII characters.
/// # Errors
/// Returns an error if the string contains non-ASCII characters.
pub(crate) fn contains_numbers(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    text.chars().any(|c| c.is_ascii_digit())
}

/// Checks if a string contains special characters.
/// # Examples
/// ```
/// use utils::contains_special_characters;
/// let text_without = "Hello, world!";
/// assert_eq!(contains_special_characters(text), false);
/// let text_with = "Hello, world@!";
/// assert_eq!(contains_special_characters(text), true);
/// ```
/// # Panics
/// Panics if the string contains non-ASCII characters.
/// # Errors
/// Returns an error if the string contains non-ASCII characters.
pub(crate) fn contains_special_characters(text: &str, special_characters: HashSet<char>) -> bool {
    if text.is_empty() {
        return false;
    }
    text.chars().any(|c| special_characters.contains(&c))
}

/// Checks if a string is a valid entity.
/// Using the configuration file, it checks if the string is alphanumeric, contains punctuation, numbers, or special characters.
/// # Examples
/// ```
/// use utils::is_valid;
/// let text = "Hello, world!";
/// assert_eq!(is_valid(config, text), true);
/// ```
pub(crate) fn is_valid(filters: &Filters, text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    // False
    if filters.alphanumeric && is_alphanumeric(text) {
        debug!("{} is not alphanumeric", text);
        return false;
    }
    if filters.punctuation && contains_punctuation(text) {
        debug!("'{}' contains punctuation", text);
        return false;
    }
    if filters.numbers && contains_numbers(text) {
        debug!("{} does not contain numbers", text);
        return false;
    }
    if filters.special_characters
        && contains_special_characters(text, filters.get_special_characters())
    {
        debug!("{} contains special characters", text);
        return false;
    }
    if filters.min_length >= 0 && text.len() < filters.min_length as usize {
        debug!("{} is too short", text);
        return false;
    }
    if filters.max_length >= 0 && text.len() > filters.max_length as usize {
        return false;
    }
    true
}

/// Get a progress bar with a custom style.
/// # Examples
/// ```
/// use utils::get_progress_bar;
/// let progress_bar = get_progress_bar(100);
/// ```
pub(crate) fn get_progress_bar(total: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(total);

    progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/blue}] {human_pos}/{human_len} ({eta})")
        .unwrap()
        .progress_chars("##-"));
    progress_bar
}
