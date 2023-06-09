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
//! - [Online tutorial](https://zu.biofan.org)
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
    clippy::multiple_crate_versions,
    clippy::struct_excessive_bools
)]

pub mod accordion;
pub mod accordion_actions;
pub mod accordion_details;
pub mod accordion_summary;
pub mod alert;
pub mod alert_title;
pub mod app_bar;
pub mod autocomplete;
pub mod avatar;
pub mod avatar_group;
pub mod backdrop;
pub mod badge;
pub mod bottom_navigation;
pub mod bottom_navigation_action;
pub mod r#box;
pub mod breadcrumbs;
pub mod button;
pub mod button_base;
pub mod button_group;
pub mod card;
pub mod card_action_area;
pub mod card_actions;
pub mod card_content;
pub mod card_header;
pub mod card_media;
pub mod checkbox;
pub mod chip;
pub mod circular_progress;
pub mod code;
pub mod collapse;
pub mod container;
pub mod css_baseline;
pub mod dialog;
pub mod dialog_actions;
pub mod dialog_content;
pub mod dialog_content_text;
pub mod dialog_title;
pub mod divider;
pub mod drawer;
pub mod fab;
pub mod fade;
pub mod filled_input;
pub mod form_control;
pub mod form_control_label;
pub mod form_group;
pub mod form_helper_text;
pub mod form_label;
pub mod grow;
pub mod icon_button;
pub mod input_label;
pub mod internal;
pub mod linear_progress;
pub mod link;
pub mod list;
pub mod list_item;
pub mod list_item_avatar;
pub mod list_item_button;
pub mod list_item_icon;
pub mod list_item_secondary_action;
pub mod list_item_text;
pub mod list_subheader;
pub mod paper;
pub mod scoped_css_baseline;
pub mod skeleton;
pub mod stack;
pub mod step;
pub mod step_button;
pub mod step_connector;
pub mod step_content;
pub mod step_icon;
pub mod step_label;
pub mod styles;
pub mod svg_icon;
pub mod switch;
pub mod switch_base;
pub mod tab;
pub mod tab_content;
pub mod table;
pub mod table_body;
pub mod table_cell;
pub mod table_container;
pub mod table_footer;
pub mod table_head;
pub mod table_pagination;
pub mod table_row;
pub mod theme_provider;
pub mod timeline;
pub mod timeline_connector;
pub mod timeline_content;
pub mod timeline_dot;
pub mod timeline_item;
pub mod timeline_opposite_content;
pub mod timeline_separator;
pub mod toggle_button;
pub mod toggle_button_group;
pub mod toolbar;
pub mod tree_item;
pub mod tree_view;
pub mod typography;
pub mod utils;
pub mod zoom;
