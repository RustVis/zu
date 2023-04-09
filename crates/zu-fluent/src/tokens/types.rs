// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

type Val = &'static str;

pub struct ColorTokens {
    pub neutral_foreground1: Val,
    pub neutral_foreground1_hover: Val,
    pub neutral_foreground1_pressed: Val,
    pub neutral_foreground1_selected: Val,
    pub neutral_foreground2: Val,
    pub neutral_foreground2_hover: Val,
    pub neutral_foreground2_pressed: Val,
    pub neutral_foreground2_selected: Val,
    pub neutral_foreground2_brand_hover: Val,
    pub neutral_foreground2_brand_pressed: Val,
    pub neutral_foreground2_brand_selected: Val,
    pub neutral_foreground3: Val,
    pub neutral_foreground3_hover: Val,
    pub neutral_foreground3_pressed: Val,
    pub neutral_foreground3_selected: Val,
    pub neutral_foreground3_brand_hover: Val,
    pub neutral_foreground3_brand_pressed: Val,
    pub neutral_foreground3_brand_selected: Val,
    pub neutral_foreground4: Val,
    pub neutral_foreground_disabled: Val,
    pub neutral_foreground_inverted_disabled: Val,
    pub brand_foreground_link: Val,
    pub brand_foreground_link_hover: Val,
    pub brand_foreground_link_pressed: Val,
    pub brand_foreground_link_selected: Val,
    pub neutral_foreground2_link: Val,
    pub neutral_foreground2_link_hover: Val,
    pub neutral_foreground2_link_pressed: Val,
    pub neutral_foreground2_link_selected: Val,
    pub compound_brand_foreground1: Val,
    pub compound_brand_foreground1_hover: Val,
    pub compound_brand_foreground1_pressed: Val,
    pub brand_foreground1: Val,
    pub brand_foreground2: Val,
    pub neutral_foreground1_static: Val,
    pub neutral_foreground_inverted: Val,
    pub neutral_foreground_inverted_hover: Val,
    pub neutral_foreground_inverted_pressed: Val,
    pub neutral_foreground_inverted_selected: Val,
    pub neutral_foreground_inverted2: Val,
    pub neutral_foreground_on_brand: Val,
    pub neutral_foreground_static_inverted: Val,
    pub neutral_foreground_inverted_link: Val,
    pub neutral_foreground_inverted_link_hover: Val,
    pub neutral_foreground_inverted_link_pressed: Val,
    pub neutral_foreground_inverted_link_selected: Val,
    pub brand_foreground_inverted: Val,
    pub brand_foreground_inverted_hover: Val,
    pub brand_foreground_inverted_pressed: Val,
    pub brand_foreground_on_light: Val,
    pub brand_foreground_on_light_hover: Val,
    pub brand_foreground_on_light_pressed: Val,
    pub brand_foreground_on_light_selected: Val,
    pub neutral_background1: Val,
    pub neutral_background1_hover: Val,
    pub neutral_background1_pressed: Val,
    pub neutral_background1_selected: Val,
    pub neutral_background2: Val,
    pub neutral_background2_hover: Val,
    pub neutral_background2_pressed: Val,
    pub neutral_background2_selected: Val,
    pub neutral_background3: Val,
    pub neutral_background3_hover: Val,
    pub neutral_background3_pressed: Val,
    pub neutral_background3_selected: Val,
    pub neutral_background4: Val,
    pub neutral_background4_hover: Val,
    pub neutral_background4_pressed: Val,
    pub neutral_background4_selected: Val,
    pub neutral_background5: Val,
    pub neutral_background5_hover: Val,
    pub neutral_background5_pressed: Val,
    pub neutral_background5_selected: Val,
    pub neutral_background6: Val,
    pub neutral_background_inverted: Val,
    pub neutral_background_static: Val,
    pub neutral_background_alpha: Val,
    pub neutral_background_alpha2: Val,
    pub subtle_background: Val,
    pub subtle_background_hover: Val,
    pub subtle_background_pressed: Val,
    pub subtle_background_selected: Val,
    pub subtle_background_light_alpha_hover: Val,
    pub subtle_background_light_alpha_pressed: Val,
    pub subtle_background_light_alpha_selected: Val,
    pub subtle_background_inverted: Val,
    pub subtle_background_inverted_hover: Val,
    pub subtle_background_inverted_pressed: Val,
    pub subtle_background_inverted_selected: Val,
    pub transparent_background: Val,
    pub transparent_background_hover: Val,
    pub transparent_background_pressed: Val,
    pub transparent_background_selected: Val,
    pub neutral_background_disabled: Val,
    pub neutral_background_inverted_disabled: Val,
    pub neutral_stencil1: Val,
    pub neutral_stencil2: Val,
    pub neutral_stencil1_alpha: Val,
    pub neutral_stencil2_alpha: Val,
    pub background_overlay: Val,
    pub scrollbar_overlay: Val,
    pub brand_background: Val,
    pub brand_background_hover: Val,
    pub brand_background_pressed: Val,
    pub brand_background_selected: Val,
    pub compound_brand_background: Val,
    pub compound_brand_background_hover: Val,
    pub compound_brand_background_pressed: Val,
    pub brand_background_static: Val,
    pub brand_background2: Val,
    pub brand_background_inverted: Val,
    pub brand_background_inverted_hover: Val,
    pub brand_background_inverted_pressed: Val,
    pub brand_background_inverted_selected: Val,
    pub neutral_stroke_accessible: Val,
    pub neutral_stroke_accessible_hover: Val,
    pub neutral_stroke_accessible_pressed: Val,
    pub neutral_stroke_accessible_selected: Val,
    pub neutral_stroke1: Val,
    pub neutral_stroke1_hover: Val,
    pub neutral_stroke1_pressed: Val,
    pub neutral_stroke1_selected: Val,
    pub neutral_stroke2: Val,
    pub neutral_stroke3: Val,
    pub neutral_stroke_on_brand: Val,
    pub neutral_stroke_on_brand2: Val,
    pub neutral_stroke_on_brand2_hover: Val,
    pub neutral_stroke_on_brand2_pressed: Val,
    pub neutral_stroke_on_brand2_selected: Val,
    pub brand_stroke1: Val,
    pub brand_stroke2: Val,
    pub compound_brand_stroke: Val,
    pub compound_brand_stroke_hover: Val,
    pub compound_brand_stroke_pressed: Val,
    pub neutral_stroke_disabled: Val,
    pub neutral_stroke_inverted_disabled: Val,
    pub transparent_stroke: Val,
    pub transparent_stroke_interactive: Val,
    pub transparent_stroke_disabled: Val,
    pub neutral_stroke_alpha: Val,
    pub stroke_focus1: Val,
    pub stroke_focus2: Val,
    pub neutral_shadow_ambient: Val,
    pub neutral_shadow_key: Val,
    pub neutral_shadow_ambient_lighter: Val,
    pub neutral_shadow_key_lighter: Val,
    pub neutral_shadow_ambient_darker: Val,
    pub neutral_shadow_key_darker: Val,
    pub brand_shadow_ambient: Val,
    pub brand_shadow_key: Val,
}

pub struct BrandVariants {
    pub b10: Val,
    pub b20: Val,
    pub b30: Val,
    pub b40: Val,
    pub b50: Val,
    pub b60: Val,
    pub b70: Val,
    pub b80: Val,
    pub b90: Val,
    pub b100: Val,
    pub b110: Val,
    pub b120: Val,
    pub b130: Val,
    pub b140: Val,
    pub b150: Val,
    pub b160: Val,
}

pub struct FontSizeTokens {
    pub s100: Val,
    pub s200: Val,
    pub s300: Val,
    pub s400: Val,
    pub s500: Val,
    pub s600: Val,
    pub s700: Val,
    pub s800: Val,
    pub s900: Val,
    pub s1000: Val,
}

pub struct LineHeightTokens {
    pub h100: Val,
    pub h200: Val,
    pub h300: Val,
    pub h400: Val,
    pub h500: Val,
    pub h600: Val,

    pub h700: Val,
    pub h800: Val,
    pub h900: Val,
    pub h1000: Val,
}

pub struct FontWeightTokens {
    pub regular: i32,
    pub medium: i32,
    pub semibold: i32,
    pub bold: i32,
}

pub struct FontFamilyTokens {
    pub base: Val,
    pub monospace: Val,
    pub numeric: Val,
}

pub enum TextAlignment {
    Inherit,
    Initial,
    Revert,
    Unset,
    Center,
    End,
    Start,
    Justify,
    Left,
    MatchParent,
    Right,
}

pub struct TextAlignments {
    pub start: TextAlignment,
    pub center: TextAlignment,
    pub end: TextAlignment,
    pub justify: TextAlignment,
}

pub struct TypographyStyle {
    pub font_family: Val,
    pub font_size: Val,
    pub font_weight: Val,
    pub line_height: Val,
}

pub struct TypographyStyles {
    pub body1: TypographyStyle,
    pub body1_strong: TypographyStyle,
    pub body1_stronger: TypographyStyle,
    pub body2: TypographyStyle,
    pub caption1: TypographyStyle,
    pub caption1_strong: TypographyStyle,
    pub caption1_stronger: TypographyStyle,
    pub caption2: TypographyStyle,
    pub caption2_strong: TypographyStyle,
    pub subtitle1: TypographyStyle,
    pub subtitle2: TypographyStyle,
    pub subtitle2_stronger: TypographyStyle,
    pub title1: TypographyStyle,
    pub title2: TypographyStyle,
    pub title3: TypographyStyle,
    pub large_title: TypographyStyle,
    pub display: TypographyStyle,
}

pub struct BorderRadiusTokens {
    pub none: Val,
    pub small: Val,
    pub medium: Val,
    pub large: Val,
    pub xlarge: Val,
    pub circular: Val,
}

pub struct StrokeWidthTokens {
    pub thin: Val,
    pub thick: Val,
    pub thicker: Val,
    pub thickest: Val,
}

pub struct SpacingTokens {
    pub none: Val,
    pub xxs: Val,
    pub xs: Val,
    pub s_nudge: Val,
    pub s: Val,
    pub m_nudge: Val,
    pub m: Val,
    pub l: Val,
    pub xl: Val,
    pub xxl: Val,
    pub xxxl: Val,
}

pub struct HorizontalSpacingTokens {
    pub none: Val,
    pub xxs: Val,
    pub xs: Val,
    pub s_nudge: Val,
    pub s: Val,
    pub m_nudge: Val,
    pub m: Val,
    pub l: Val,
    pub xl: Val,
    pub xxl: Val,
    pub xxxl: Val,
}

pub struct VerticalSpacingTokens {
    pub none: Val,
    pub xxs: Val,
    pub xs: Val,
    pub s_nudge: Val,
    pub s: Val,
    pub m_nudge: Val,
    pub m: Val,
    pub l: Val,
    pub xl: Val,
    pub xxl: Val,
    pub xxxl: Val,
}

pub struct DurationTokens {
    pub ultra_fast: Val,
    pub faster: Val,
    pub fast: Val,
    pub normal: Val,
    pub slow: Val,
    pub slower: Val,
    pub ultra_slow: Val,
}

pub struct CurveTokens {
    pub accelerate_max: Val,
    pub accelerate_mid: Val,
    pub accelerate_min: Val,
    pub decelerate_max: Val,
    pub decelerate_mid: Val,
    pub decelerate_min: Val,
    pub easy_ease_max: Val,
    pub easy_ease: Val,
    pub linear: Val,
}

pub struct ShadowTokens {
    pub s2: Val,
    pub s4: Val,
    pub s8: Val,
    pub s16: Val,
    pub s28: Val,
    pub s64: Val,
}

pub struct ShadowBrandTokens {
    pub s2: Val,
    pub s4: Val,
    pub s8: Val,
    pub s16: Val,
    pub s28: Val,
    pub s64: Val,
}
