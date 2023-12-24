// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    #[default]
    OnClick,
    OnMouseDown,
    OnMouseUp,
    OnPointerDown,
    OnPointerUp,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TouchEventType {
    #[default]
    OnTouchEnd,
    OnTouchStart,
}
