// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![allow(clippy::module_name_repetitions)]

pub mod compute_coords;
pub mod compute_position;
pub mod detect_overflow;
mod middleware;
pub mod middlewares;
pub mod platform;
pub mod types;
pub mod vanilla;
