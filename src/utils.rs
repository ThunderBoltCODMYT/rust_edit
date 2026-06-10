// utils.rs:
/*
* MIT License

Copyright (c) 2026 ThunderBoltCODMYT

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

pub fn default_filename() -> String {
    "untitled.txt".to_string()
}

// we are going to use conditional compilation to cover mostly every embedded && desktop OS
#[cfg(target_os = "windows")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

    if name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.ends_with(' ') || name.ends_with('.') {
        return false;
    }

    let upper = name.to_uppercase();

    let reserved = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    if reserved.contains(&upper.as_str()) {
        return false;
    }

    if name.len() - 1 > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "linux")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() - 1 > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "macos")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0', ':'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "freebsd")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['\0', '/'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "openbsd")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['\0', '/'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "netbsd")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "dragonfly")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "haiku")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "redox")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "hurd")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    true
}

#[cfg(target_os = "illumos")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "solaris")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "aix")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "espidf")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', ':', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.ends_with('.') || name.ends_with("..") {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "horizon")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0', ':', '*', '?', '\"', '<', '>', '|'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "vita")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0', ':', '*', '?', '\"', '<', '>', '|'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "xous")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "hermit")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "l4re")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

#[cfg(target_os = "nto")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    // NTO also forbids some hexadecimal values, you cant store those in a char array, (chars are also 4 bytes, unicode for rust), you usually store them in a u32
    // the forbidden values are: 0x7F and 0xFF , because 0x7F is the DEL control character on QNX systems, and 0xFF because in C, its used to represent EOF, as for End Of File
    let bytes = name.as_bytes();

    for &b in bytes {
        if b == 0x7F || b == 0xFF {
            return false;
        }
    }

    if name.len() > 505 {
        return false;
    }

    true
}

#[cfg(target_os = "teeos")]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}

// Put this at the bottom to catch everything else
#[cfg(not(any(
    target_os = "windows", target_os = "linux", target_os = "macos",
    target_os = "freebsd", target_os = "openbsd", target_os = "netbsd",
    target_os = "dragonfly", target_os = "haiku", target_os = "redox",
    target_os = "hurd", target_os = "illumos", target_os = "solaris",
    target_os = "aix", target_os = "espidf", target_os = "horizon",
    target_os = "vita", target_os = "xous", target_os = "hermit",
    target_os = "l4re", target_os = "nto", target_os = "teeos"
)))]
pub fn is_valid_filename(name: &str) -> bool {
    let invalid = ['/', '\0'];

    if name.is_empty() || name.chars().any(|c| invalid.contains(&c)) {
        return false;
    }

    if name.len() > 255 {
        return false;
    }

    true
}
