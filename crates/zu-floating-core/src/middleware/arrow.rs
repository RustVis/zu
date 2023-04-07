// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::fmt;
use std::rc::Rc;

use crate::traits::Element;
use crate::types::Padding;

#[derive(Clone)]
pub struct ArrowOption {
    /// The arrow element to be positioned.
    pub element: Rc<dyn Element>,

    /// The padding between the arrow element and the floating element edges.
    ///
    /// Useful when the floating element has round corners.
    pub padding: Option<Padding>,
}

impl fmt::Debug for ArrowOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArrowOption")
            .field("padding", &self.padding)
            .finish()
    }
}
