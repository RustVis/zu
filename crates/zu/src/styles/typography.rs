// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// Get css variables of typography.
pub trait TypographyVariables {
    fn component(&self) -> &'static str;

    // TODO(Shaohua): Replace with macro
    fn gen(&self, variant: &str) -> String {
        format!("--zu-typography-{}-{}", self.component(), variant)
    }

    fn font_family(&self) -> String {
        self.gen("fontFamily")
    }

    fn font_weight(&self) -> String {
        self.gen("fontWeight")
    }

    fn font_size(&self) -> String {
        self.gen("fontSize")
    }

    fn line_height(&self) -> String {
        self.gen("lineHeight")
    }

    fn letter_spacing(&self) -> String {
        self.gen("letterSpacing")
    }
}
