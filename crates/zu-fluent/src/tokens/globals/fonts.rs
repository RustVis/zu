// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub use crate::tokens::types::{
    FontFamilyTokens, FontSizeTokens, FontWeightTokens, LineHeightTokens,
};

pub const FONT_SIZES: FontSizeTokens = FontSizeTokens {
    s100: "10px",
    s200: "12px",
    s300: "14px",
    s400: "16px",
    s500: "20px",
    s600: "24px",
    s700: "28px",
    s800: "32px",
    s900: "40px",
    s1000: "68px",
};

pub const LINE_HEIGHTS: LineHeightTokens = LineHeightTokens {
    h100: "14px",
    h200: "16px",
    h300: "20px",
    h400: "22px",
    h500: "28px",
    h600: "32px",

    h700: "36px",
    h800: "40px",
    h900: "52px",
    h1000: "92px",
};

pub const FONT_WEIGHTS: FontWeightTokens = FontWeightTokens {
    regular: 400,
    medium: 500,
    semi_bold: 600,
    bold: 700,
};

pub const FONT_FAMILIES: FontFamilyTokens = FontFamilyTokens {
  base: "'Segoe UI', 'Segoe UI Web (West European)', -apple-system, BlinkMacSystemFont, Roboto, 'Helvetica Neue', sans-serif",
  monospace: "Consolas, 'Courier New', Courier, monospace",
  numeric: "Bahnschrift, 'Segoe UI', 'Segoe UI Web (West European)', -apple-system, BlinkMacSystemFont, Roboto, 'Helvetica Neue', sans-serif",
};
