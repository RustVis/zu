// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

pub trait Scalar {
    fn nearly_equal(self, other: Self) -> bool;
    fn nearly_zero(self) -> bool;
}

#[allow(clippy::cast_precision_loss)]
const NEARLY_ZERO_F32: f32 = 1.0 / (1 << 12) as f32;
#[allow(clippy::cast_precision_loss)]
const NEARLY_ZERO_F64: f64 = 1.0 / (1 << 18) as f64;

impl Scalar for f32 {
    fn nearly_equal(self, other: Self) -> bool {
        (self - other).abs() <= NEARLY_ZERO_F32
    }

    fn nearly_zero(self) -> bool {
        self.abs() <= NEARLY_ZERO_F32
    }
}

impl Scalar for f64 {
    fn nearly_equal(self, other: Self) -> bool {
        (self - other).abs() <= NEARLY_ZERO_F64
    }

    fn nearly_zero(self) -> bool {
        self.abs() <= NEARLY_ZERO_F64
    }
}
