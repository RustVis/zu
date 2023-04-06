// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Impl Platform trait to support new platform environment.
pub trait Platform: Sized {
    fn dimensions(&self) -> i32;

    /// Returns true if layout direction is Right-To-Left.
    fn is_rtl(&self) -> bool;

    fn clipping_rect(&self) -> (f32, f32);
}
