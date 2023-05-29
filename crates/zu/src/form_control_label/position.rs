// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::position::Position;

pub const fn css_class(position: Position) -> &'static str {
    match position {
        Position::Start => "ZuFormControlLabel-labelPlacementStart",
        Position::End => "ZuFormControlLabel-labelPlacementEnd",
        Position::Top => "ZuFormControlLabel-labelPlacementTop",
        Position::Bottom => "ZuFormControlLabel-labelPlacementBottom",
    }
}
