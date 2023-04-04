// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
// TODO(Shaohua): Update syn to 2.0
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]

pub mod components;
pub mod util;
