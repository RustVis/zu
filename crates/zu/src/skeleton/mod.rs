// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod animation;
mod variant;

use yew::{classes, function_component, html, AttrValue, Classes, Html, Properties};

use crate::styles::CssClass;

// Re-export
pub use animation::Animation;
pub use variant::Variant;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub animation: Animation,

    #[prop_or_default]
    pub classes: Classes,

    /// The type of content that will be rendered.
    #[prop_or_default]
    pub variant: Variant,

    /// Width of the skeleton.
    #[prop_or_default]
    pub width: i32,

    /// Height of the skeleton.
    #[prop_or_default]
    pub height: i32,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Skeleton)]
pub fn skeleton(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuSkeleton-root",
        props.animation.css_class(),
        props.variant.css_class(),
        props.classes.clone(),
    );

    let style = [
        props.style.as_str(),
        if props.width > 0 {
            format!("width: {}px", props.width)
        } else {
            String::new()
        }
        .as_str(),
        if props.height > 0 {
            format!("height: {}px", props.height)
        } else {
            String::new()
        }
        .as_str(),
    ]
    .join(";");

    html! {
        <span class={root_cls} style={style}>
        </span>
    }
}
