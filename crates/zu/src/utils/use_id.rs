// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use std::sync::atomic::{AtomicU32, Ordering};

pub fn global_id() -> String {
    static ID: AtomicU32 = AtomicU32::new(1);
    let id = ID.fetch_add(1, Ordering::Relaxed);
    format!("zu-{id}")
}
