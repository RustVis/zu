// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! ![Build status](https://github.com/RustVis/zu/actions/workflows/rust.yml/badge.svg)
//! [![Latest version](https://img.shields.io/crates/v/zu.svg)](https://crates.io/crates/zu)
//! [![Documentation](https://docs.rs/zu/badge.svg)](https://docs.rs/zu)
//! ![Minimum rustc version](https://img.shields.io/badge/rustc-1.56+-yellow.svg)
//! ![License](https://img.shields.io/crates/l/zu.svg)
//!
//! [Yew](https://yew.rs) web components, implementing Material Design.
//!
//! This UI library is heavily inspired by [material ui](https://github.com/mui/material-ui).
//!
//! - [Documentation](https://docs.rs/zu)
//! - [Release notes](https://github.com/RustVis/zu/releases)
//!
//! ## Usage
//! Add this to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! zu = "0.2"
//! ```
//!
//! Or using `cargo add zu` command.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![allow(
    dead_code,
    clippy::let_underscore_untyped,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions
)]

pub mod circular_progress;
pub mod skeleton;
pub mod styles;
pub mod theme_provider;
