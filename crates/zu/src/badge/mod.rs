// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod anchor_origin;
mod color;
mod overlap;
mod variant;

use yew::{classes, function_component, html, Children, Html, Properties};

use crate::styles::CssClass;

// Re-export property items.
pub use anchor_origin::AnchorOrigin;
pub use color::Color;
pub use overlap::Overlap;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub anchor_origin: AnchorOrigin,

    /// The content rendered within the badge.
    #[prop_or_default]
    pub content: Children,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    // TODO(Shaohua): Add Color::Default
    pub color: Color,

    #[prop_or_default]
    pub component: String,

    /// If true, the badge is invisible.
    pub invisible: bool,

    /// Max count to show.
    pub max: i32,

    /// Wrapped shape the badge should overlap.
    #[prop_or_default]
    pub overlap: Overlap,

    /// Controls whether the badge is hidden when badgeContent is zero.
    #[prop_or(false)]
    pub show_zero: bool,

    /// The variant to use.
    pub variant: Variant,
}

#[function_component(Badge)]
pub fn badge(props: &Props) -> Html {
    let cls_list = vec![
        "ZuBadge-badge",
        props.variant.css_class(),
        if props.invisible {
            "ZuBadge-invisible"
        } else {
            ""
        },
        props.overlap.css_class(),
        props.color.css_class(),
    ];
    let cls = classes!(cls_list);

    let component = if props.component.is_empty() {
        "span".to_owned()
    } else {
        props.component.clone()
    };

    html! {
        <@{component} class="ZuBadge-root">
            <span class={cls}>
            </span>
        </@>
    }
}
