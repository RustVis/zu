// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

//! # Zu icon material
//! ![Build status](https://github.com/RustVis/zu/actions/workflows/rust.yml/badge.svg)
//! [![Latest version](https://img.shields.io/crates/v/zuicon-material.svg)](https://crates.io/crates/zuicon-material)
//! [![Documentation](https://docs.rs/zuicon-material/badge.svg)](https://docs.rs/zuicon-material)
//! ![Minimum rustc version](https://img.shields.io/badge/rustc-1.56+-yellow.svg)
//! ![License](https://img.shields.io/crates/l/zuicon-material.svg)
//!
//! Wrapper of material design icons for yew framework.
//!
//! Material icons are placed in different themes:
//! 1. baseline (default)
//! 2. outline
//! 3. round
//! 4. twotone
//! 5. sharp
//!
//! - [Documentation](https://docs.rs/zuicon-material)
//! - [Online tutorial](https://zu.biofan.org/material-icons)
//!
//!
//! ## How to use
//! First add this to `Cargo.toml`:
//! ```toml
//! [dependencies.zuicon-material]
//! version = "0.2"
//! features = [
//!   "Home",
//!   "Email",
//! ]
//! ```
//!
//! `Home` and `Email` are available.
//!
//! Then import specific icons in yew components:
//! `use zuicon_material::{email::Email, home::Home};`
//!
//! ## Related projects
//! - [MUI Material Icons][icons-material]
//!
//! [icons-material]: https://github.com/mui/material-ui/tree/master/packages/mui-icons-material

mod icons;
pub use icons::*;

mod custom;
pub use custom::*;
