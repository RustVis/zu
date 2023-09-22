// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    OnClick,
    OnMouseDown,
    OnMouseUp,
    OnPointerDown,
    OnPointerUp,
}

impl Default for MouseEventType {
    fn default() -> Self {
        Self::OnClick
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchEventType {
    OnTouchEnd,
    OnTouchStart,
}

impl Default for TouchEventType {
    fn default() -> Self {
        Self::OnTouchEnd
    }
}
