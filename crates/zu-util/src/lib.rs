// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

//! # Zu util
//!
//! ![Build status](https://github.com/RustVis/zu/actions/workflows/rust.yml/badge.svg)
//! [![Latest version](https://img.shields.io/crates/v/zu-util.svg)](https://crates.io/crates/zu-util)
//! [![Documentation](https://docs.rs/zu-util/badge.svg)](https://docs.rs/zu-util)
//! ![Minimum rustc version](https://img.shields.io/badge/rustc-1.56+-yellow.svg)
//! ![License](https://img.shields.io/crates/l/zu-util.svg)
//!
//! Shared utilities for [zu](https://github.com/RustVis/zu) web component.
//!
//! - [Documentation](https://docs.rs/zu-util)
//! - [Release notes](https://github.com/RustVis/zu-util/releases)
//!
//! ## Usage
//! Add this to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! zu-util = "0.1"
//! ```
//!
//! Or using `cargo add zu-util` command.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]

#[cfg(feature = "icon")]
pub mod icon;

pub mod cmp;
pub mod image_future;
pub mod name;
pub mod prop;
pub mod scalar;
