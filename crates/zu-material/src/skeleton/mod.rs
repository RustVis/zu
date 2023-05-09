// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html, Properties};

/// The type of content that will be rendered.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Variant {
    Text,
    Circular,
    Rect,
    Rounded,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Animation {
    Pulse,
    Wave,
    None,
}

impl Default for Animation {
    fn default() -> Self {
        Self::Pulse
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub animation: Animation,

    /// The type of content that will be rendered.
    #[prop_or_default]
    pub variant: Variant,

    /// Width of the skeleton.
    #[prop_or_default]
    pub width: i32,

    /// Height of the skeleton.
    #[prop_or_default]
    pub height: i32,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            animation: Animation::default(),
            variant: Variant::default(),
            width: 100,
            height: 100,
        }
    }
}

#[function_component(Skeleton)]
pub fn skeleton(_props: &Props) -> Html {
    html! {}
}
