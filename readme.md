# limace

**`limace`** is a fast and minimal slugification utility for Rust. It converts strings into clean, URL-friendly slugs using ASCII characters only. It handles Unicode characters via transliteration and supports customizable separator characters.

## Features

- Transliterates Unicode to ASCII using [`deunicode`](https://crates.io/crates/deunicode)
- Strips punctuation and special characters
- Converts to lowercase
- Customizable separator (default is `-`)
- No heap allocations beyond the result `String`

## Example

```rust
use limace::Slugifier;

fn main() {
    let slugifier = Slugifier::default();

    assert_eq!(slugifier.slugify("Hello, World!"), "hello-world");
    assert_eq!(slugifier.slugify("Crème brûlée"), "creme-brulee");

    let custom = Slugifier::default().with_separator('_');
    assert_eq!(custom.slugify("Hello, World!"), "hello_world");
}
````

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
limace = "0.1"
```

Then use it in your code:

```rust
use limace::Slugifier;

let slug = Slugifier::default().slugify("Rust 2024: Fast & Fearless!");
assert_eq!(slug, "rust-2024-fast-fearless");
```

### Custom separator

You can change the separator using `with_separator()`:

```rust
use limace::Slugifier;

let slug = Slugifier::default()
    .with_separator('_')
    .slugify("Hello, World!");
assert_eq!(slug, "hello_world");
```

## How it works

1. Unicode characters are transliterated to ASCII using `deunicode_char()`.
2. Uppercase letters are manually lowercased.
3. Alphanumeric ASCII characters are kept.
4. All other characters are replaced by a separator.
5. Consecutive separators are collapsed into one.

## License

Licensed under MIT
