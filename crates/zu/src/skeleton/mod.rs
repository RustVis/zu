// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod animation;
mod variant;

use yew::{classes, function_component, html, AttrValue, Html, Properties};

use crate::styles::CssClass;

// Re-export
pub use animation::Animation;
pub use variant::Variant;

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

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Skeleton)]
pub fn skeleton(props: &Props) -> Html {
    let cls = classes!(
        "ZuSkeleton-root",
        props.animation.css_class(),
        props.variant.css_class()
    );
    let mut styles = vec![props.style.as_str().to_owned()];
    if props.width > 0 {
        styles.push(format!("width: {}px", props.width));
    };
    if props.height > 0 {
        styles.push(format!("height: {}px", props.height));
    }
    let style = styles.join(";");

    html! {
        <span class={cls} style={style}>
        </span>
    }
}
