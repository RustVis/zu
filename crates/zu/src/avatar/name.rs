// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[must_use]
pub fn name_to_color(name: &str) -> String {
    let mut s = DefaultHasher::new();
    name.hash(&mut s);
    let u64_hash = s.finish();

    let mut value = [0u64; 3];
    for (i, item) in value.iter_mut().enumerate() {
        *item = (u64_hash >> (i * 8)) & 0xff;
    }

    let color = format!("#{:02x}{:02x}{:02x}", value[0], value[1], value[2]);
    log::info!("color: {color:?}");
    color
}

#[must_use]
pub fn abbr_name(name: &str) -> String {
    let parts = name.split(' ');
    parts
        .into_iter()
        .filter_map(|part| part.chars().next())
        .collect::<String>()
}
