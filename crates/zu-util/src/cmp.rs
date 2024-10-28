// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use float_cmp::ApproxEq;

/// Compares two floating point values and returns true if they are considered equal,
/// otherwise false.
#[inline]
#[must_use]
pub fn fuzzy_compare(p1: f64, p2: f64) -> bool {
    p1.approx_eq(p2, (0.0, 1))
}
