// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, Html, Properties};

use crate::util::CssClass;

const ROOT_CLS: &str = "ZuSkeleton-root";

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

impl CssClass for Variant {
    fn to_cls(&self) -> &'static str {
        match self {
            Self::Text => "ZuSkeleton-text",
            Self::Circular => "ZuSkeleton-circular",
            Self::Rect => "ZuSkeleton-rect",
            Self::Rounded => "ZuSkeleton-rounded",
        }
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

impl CssClass for Animation {
    fn to_cls(&self) -> &'static str {
        match self {
            Self::Pulse => "ZuSkeleton-pulse",
            Self::Wave => "ZuSkeleton-wave",
            Self::None => "",
        }
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
    #[prop_or(100)]
    pub width: i32,

    /// Height of the skeleton.
    #[prop_or(100)]
    pub height: i32,
}

#[function_component(Skeleton)]
pub fn skeleton(props: &Props) -> Html {
    let cls = classes!(ROOT_CLS, props.animation.to_cls(), props.variant.to_cls());
    let style = format!("width: {}px; height: {}px;", props.width, props.height);

    html! {
        <span class={ cls } style={ style }>
        </span>
    }
}
