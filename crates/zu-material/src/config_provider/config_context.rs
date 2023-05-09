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
    fn get_prefix_class(&self, suffix_class: &str, customize_prefix_class: Option<&str>) -> String;
}

impl ConfigContext for ConfigConsumer {
    fn direction(&self) -> DirectionType {
        self.direction
    }

    fn get_prefix_class(&self, suffix_class: &str, customize_prefix_class: Option<&str>) -> String {
        default_get_prefix_class(suffix_class, customize_prefix_class)
    }
}

#[must_use]
pub fn default_get_prefix_class(
    suffix_class: &str,
    customize_prefix_class: Option<&str>,
) -> String {
    if let Some(customize_prefix_class) = customize_prefix_class {
        if !customize_prefix_class.is_empty() {
            return customize_prefix_class.to_string();
        }
    }
    if suffix_class.is_empty() {
        return "zu".to_string();
    }
    format!("zu-{suffix_class}")
}
