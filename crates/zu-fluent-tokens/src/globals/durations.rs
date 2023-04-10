// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by MIT License that can be found
// in the LICENSE file.

use crate::types::DurationTokens;

pub const DURATIONS: DurationTokens = DurationTokens {
    ultra_fast: "50ms",
    faster: "100ms",
    fast: "150ms",
    normal: "200ms",
    slow: "300ms",
    slower: "400ms",
    ultra_slow: "500ms",
};
