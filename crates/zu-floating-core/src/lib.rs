// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that
// can be found in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![allow(clippy::module_name_repetitions)]

pub mod detect_overflow;
pub mod middleware;
pub mod middlewares;
pub mod types;
pub mod vanilla;

// mod compute_coords;
// pub use compute_coords::compute_coords_from_placement;
// mod compute_position;
// pub use compute_position::compute_position;
