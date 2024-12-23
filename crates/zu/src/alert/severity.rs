// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::severity::Severity;

#[must_use]
pub const fn severity_class(severity: Severity) -> &'static str {
    match severity {
        Severity::Success => "ZuAlert-success",
        Severity::Info => "ZuAlert-info",
        Severity::Warning => "ZuAlert-warning",
        Severity::Error => "ZuAlert-error",
    }
}
