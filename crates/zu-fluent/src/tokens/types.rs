// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

type Val = &'static str;

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

pub struct BorderRadiusTokens {
    pub none: Val,
    pub small: Val,
    pub medium: Val,
    pub large: Val,
    pub xlarge: Val,
    pub circular: Val,
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
