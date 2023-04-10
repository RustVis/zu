// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by MIT License that can be found
// in the LICENSE file.

use crate::types::{HorizontalSpacingTokens, SpacingTokens, VerticalSpacingTokens};

// Intentionally not pubed! Use horizontalSpacings and verticalSpacings instead.
const SPACINGS: SpacingTokens = SpacingTokens {
    none: "0",
    xxs: "2px",
    xs: "4px",
    s_nudge: "6px",
    s: "8px",
    m_nudge: "10px",
    m: "12px",
    l: "16px",
    xl: "20px",
    xxl: "24px",
    xxxl: "32px",
};

pub const HORIZONTAL_SPACINGS: HorizontalSpacingTokens = HorizontalSpacingTokens {
    none: SPACINGS.none,
    xxs: SPACINGS.xxs,
    xs: SPACINGS.xs,
    s_nudge: SPACINGS.s_nudge,
    s: SPACINGS.s,
    m_nudge: SPACINGS.m_nudge,
    m: SPACINGS.m,
    l: SPACINGS.l,
    xl: SPACINGS.xl,
    xxl: SPACINGS.xxl,
    xxxl: SPACINGS.xxxl,
};

pub const VERTICAL_SPACINGS: VerticalSpacingTokens = VerticalSpacingTokens {
    none: SPACINGS.none,
    xxs: SPACINGS.xxs,
    xs: SPACINGS.xs,
    s_nudge: SPACINGS.s_nudge,
    s: SPACINGS.s,
    m_nudge: SPACINGS.m_nudge,
    m: SPACINGS.m,
    l: SPACINGS.l,
    xl: SPACINGS.xl,
    xxl: SPACINGS.xxl,
    xxxl: SPACINGS.xxxl,
};
