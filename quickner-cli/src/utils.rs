// quickner
//
// NER tool for quick and simple NER annotation
// Copyright (C) 2023, Omar MHAIMDAT
//
// Licensed under Mozilla Public License 2.0
//
use std::collections::HashSet;

use indicatif::{ProgressBar, ProgressStyle};

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
