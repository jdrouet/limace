use deunicode::deunicode_char;

/// A utility for converting strings into URL-friendly slugs.
///
/// A `Slugifier` converts arbitrary input strings into slugs composed of lowercase ASCII
/// alphanumeric characters, separated by a customizable character (default is `-`).
///
/// # Examples
///
/// ```
/// use limace::Slugifier;
///
/// let slug = Slugifier::default().slugify("Hello, World!");
/// assert_eq!(slug, "hello-world");
///
/// let custom = Slugifier::default().with_separator('_');
/// assert_eq!(custom.slugify("Hello, World!"), "hello_world");
/// ```
#[derive(Clone, Debug)]
pub struct Slugifier {
    separator: char,
}

impl Default for Slugifier {
    /// Creates a default `Slugifier` with `-` as the separator.
    fn default() -> Self {
        Self { separator: '-' }
    }
}

impl Slugifier {
    /// Sets the separator character to be used in the slug.
    ///
    /// # Examples
    ///
    /// ```
    /// use limace::Slugifier;
    ///
    /// let mut slugifier = Slugifier::default();
    /// slugifier.set_separator('_');
    /// ```
    pub fn set_separator(&mut self, value: char) {
        self.separator = value;
    }

    /// Returns a new `Slugifier` with the specified separator character.
    ///
    /// # Examples
    ///
    /// ```
    /// use limace::Slugifier;
    ///
    /// let slugifier = Slugifier::default().with_separator('_');
    /// ```
    pub fn with_separator(mut self, value: char) -> Self {
        self.set_separator(value);
        self
    }

    /// Converts the input into a slug string using the current separator and rules.
    ///
    /// # Rules
    /// - Uppercase letters are converted to lowercase.
    /// - Unicode characters are transliterated to ASCII using `deunicode`.
    /// - All non-alphanumeric characters are converted to the separator.
    /// - Consecutive non-alphanumerics do not produce repeated separators.
    ///
    /// # Examples
    ///
    /// ```
    /// use limace::Slugifier;
    ///
    /// let slugifier = Slugifier::default();
    /// assert_eq!(slugifier.slugify("Crème brûlée!"), "creme-brulee");
    /// ```
    pub fn slugify(&self, input: impl AsRef<str>) -> String {
        let value = input.as_ref();
        let mut writer = SlugWriter::new(self, value.len());
        writer.push_str(value);
        writer.into_inner()
    }
}

struct SlugWriter<'a> {
    options: &'a Slugifier,
    buffer: String,
    previous_separator: bool,
}

impl<'a> SlugWriter<'a> {
    fn new(options: &'a Slugifier, size: usize) -> Self {
        Self {
            options,
            buffer: String::with_capacity(size),
            // to avoid leading separator
            previous_separator: true,
        }
    }

    fn push_separator(&mut self) {
        if !self.previous_separator {
            self.buffer.push(self.options.separator);
            self.previous_separator = true;
        }
    }

    fn push_char(&mut self, value: char) {
        match value {
            'a'..='z' | '0'..='9' => {
                self.previous_separator = false;
                self.buffer.push(value);
            }

            'A'..='Z' => {
                self.previous_separator = false;

                // Manual lowercasing as Rust to_lowercase() is unicode aware and therefore much slower
                let value = (value as u8) - b'A' + b'a';
                self.buffer.push(value as char);
            }

            _ => {
                self.push_separator();
            }
        }
    }

    fn push_str(&mut self, value: &str) {
        for c in value.chars() {
            match deunicode_char(c) {
                Some(value) => value.chars().for_each(|uc| self.push_char(uc)),
                None => self.push_separator(),
            }
        }
    }

    fn into_inner(mut self) -> String {
        if self.buffer.ends_with(self.options.separator) {
            self.buffer.pop();
        }
        self.buffer.shrink_to_fit();
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_handle_basic_slugification() {
        let slugifier = Slugifier::default();
        assert_eq!(slugifier.slugify("Hello, World!"), "hello-world");
    }

    #[test]
    fn should_handle_unicode_transliteration() {
        let slugifier = Slugifier::default();
        assert_eq!(slugifier.slugify("Crème brûlée"), "creme-brulee");
    }

    #[test]
    fn should_handle_custom_separator() {
        let slugifier = Slugifier::default().with_separator('_');
        assert_eq!(slugifier.slugify("Hello, World!"), "hello_world");
    }

    #[test]
    fn should_handle_multiple_non_alphanumerics() {
        let slugifier = Slugifier::default();
        assert_eq!(slugifier.slugify("Hello---World!!"), "hello-world");
    }

    #[test]
    fn should_handle_leading_and_trailing_specials() {
        let slugifier = Slugifier::default();
        assert_eq!(slugifier.slugify("...Hello World..."), "hello-world");
    }

    #[test]
    fn should_convert_empty_input() {
        let slugifier = Slugifier::default();
        assert_eq!(slugifier.slugify(""), "");
    }

    #[test]
    fn should_handle_numbers_and_letters() {
        let slugifier = Slugifier::default();
        assert_eq!(slugifier.slugify("Rust 2024"), "rust-2024");
    }

    #[test]
    fn should_be_idempotent() {
        let slugifier = Slugifier::default();
        let once = slugifier.slugify("Hello, World!");
        let twice = slugifier.slugify(&once);
        assert_eq!(once, twice);
    }
}
