# Windows Theme Color Changer 🎨

A simple Rust application to switch between Dark and Light themes on Windows.

## Description 📝

This Rust program allows users to change their Windows theme color between Dark and Light modes. It works by modifying the Windows Registry settings. Because there are some windows users can't change the theme color.

## How It Works 🔧

Upon running the application, the user is prompted to choose between Dark Mode (0) and Light Mode (1). Based on the input, the application modifies the registry key `SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize` to switch the theme.

## Usage 💻

1. Run the program.
2. You will see a welcome message and options:
    - `0` for Dark Mode
    - `1` for Light Mode
3. Enter your choice and the theme will be updated accordingly.

## Code Snippet 📌

```rust
extern crate winreg;
use std::io::{self, Write};
use winreg::enums::*;
use winreg::RegKey;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ASCII Art and Introduction
    // [ASCII Art Skipped in README for brevity]

    print!("{}", "Enter your choice: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let number = input.trim().parse::<u32>();
    match number {
        Ok(0) => {
            // Registry modification for Dark Mode
        },
        Ok(1) => {
            // Registry modification for Light Mode
        },
        _ => {
            // Error handling for invalid input
        }
    };

    Ok(())
}
```

## Note 🚨

- This program modifies the Windows Registry. Use it at your own risk.
- Ensure you have the necessary permissions to modify the registry.

---

Made with ❤️ by AbuJamal (Iyad Almsara)
