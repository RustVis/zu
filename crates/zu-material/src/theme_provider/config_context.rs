// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionType {
    LeftToRight,
    RightToLeft,
}

pub struct ConfigConsumer {
    direction: DirectionType,
}

pub trait ConfigContext {
    fn direction(&self) -> DirectionType;
}

impl ConfigContext for ConfigConsumer {
    fn direction(&self) -> DirectionType {
        self.direction
    }
}
