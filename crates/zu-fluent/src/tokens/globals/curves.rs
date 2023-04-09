// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::tokens::types::CurveTokens;

pub const CURVES: CurveTokens = CurveTokens {
    accelerate_max: "cubic-bezier(1,0,1,1)",
    accelerate_mid: "cubic-bezier(0.7,0,1,0.5)",
    accelerate_min: "cubic-bezier(0.8,0,1,1)",
    decelerate_max: "cubic-bezier(0,0,0,1)",
    decelerate_mid: "cubic-bezier(0.1,0.9,0.2,1)",
    decelerate_min: "cubic-bezier(0.33,0,0.1,1)",
    easy_ease_max: "cubic-bezier(0.8,0,0.1,1)",
    easy_ease: "cubic-bezier(0.33,0,0.67,1)",
    linear: "cubic-bezier(0,0,1,1)",
};
