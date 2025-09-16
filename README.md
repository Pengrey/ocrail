# OCRail
Rust implementation of windows OCR to extract text from the wallpaper

## Overview

This library allows the retrieval of text present within the computer wallpaper for later comparison between this and a given value.

## Usage

### Basic Example

Add this to your `Cargo.toml`:

```toml
[dependencies]
ocrail = { git = "https://github.com/Pengrey/ocrail.git", branch = "main" }
```

```rust
use ocrail;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_text = "MALDEV ACADEMY";
    println!("[*] Checking if '{}' is in the wallpaper...", search_text);

    if ocrail::get_text_in_wallpaper()?.contains(search_text) {
        println!("[+] Text '{}' found in wallpaper.", search_text);
    } else {
        println!("[-] Text '{}' not found in wallpaper.", search_text);
    }

    Ok(())
}
```

### Library Functions

- `get_text_in_wallpaper`: Retrieves the text present within the wallpaper for later comparisson

## Requirements

- Rust toolchain
- `windows` crate with appropriate features

## License

This project is provided as-is for educational purposes. Please refer to the original blog post for attribution and additional context.
