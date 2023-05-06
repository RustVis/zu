// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub mod amber;
pub mod blue;
pub mod blue_grey;
pub mod brown;
pub mod common;

#[derive(Debug, Clone)]
pub struct Color {
    a50: &'static str,
    a100: &'static str,
    a200: &'static str,
    a300: &'static str,
    a400: &'static str,
    a500: &'static str,
    a600: &'static str,
    a700: &'static str,
    a800: &'static str,
    a900: &'static str,
    a1000: &'static str,
    a2000: &'static str,
    a4000: &'static str,
    a7000: &'static str,
}

#[derive(Debug, Clone)]
pub struct CommonColor {
    black: &'static str,
    white: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteMode {
    Light,
    Dark,
}
