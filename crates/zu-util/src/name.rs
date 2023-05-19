// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate RGB color from name.
#[must_use]
pub fn to_color(name: &str) -> String {
    let mut s = DefaultHasher::new();
    name.hash(&mut s);
    let u64_hash = s.finish();

    let mut value = [0u8; 3];
    for (i, item) in value.iter_mut().enumerate() {
        *item = ((u64_hash >> (i * 8)) & 0xff) as u8;
    }

    let color = format!("#{:02x}{:02x}{:02x}", value[0], value[1], value[2]);
    color
}

/// Generate name abbreviation.
#[must_use]
pub fn abbreviate(name: &str) -> String {
    let parts = name.split(' ');
    parts
        .into_iter()
        .filter_map(|part| part.chars().next())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::{abbreviate, to_color};

    #[test]
    fn test_to_color() {
        assert_eq!(to_color("Kent Dodds").as_str(), "#be6bee");
        assert_eq!(to_color("Jed").as_str(), "#1dfe68");
        assert_eq!(to_color("Tim Neutkens").as_str(), "#627a8d");
    }

    #[test]
    fn test_abbreviate() {
        assert_eq!(abbreviate("Kent Dodds").as_str(), "KD");
        assert_eq!(abbreviate("Jed").as_str(), "J");
        assert_eq!(abbreviate("Tim Neutkens").as_str(), "TN");
    }
}
