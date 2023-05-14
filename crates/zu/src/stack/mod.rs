// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::direction::Direction;
use crate::styles::spacing::Spacing;
use crate::styles::CssClass;
use yew::virtual_dom::VNode;
use yew::{classes, function_component, html, Children, Html, Properties};

#[must_use]
pub const fn spacing_cls(spacing: Spacing) -> &'static str {
    match spacing {
        Spacing::None => "ZuStack-spacing-none",
        Spacing::XXSmall => "ZuStack-spacing-xxs",
        Spacing::XSmall => "ZuStack-spacing-xs",
        Spacing::SmallNudge => "ZuStack-spacing-sNudge",
        Spacing::Small => "ZuStack-spacing-s",
        Spacing::MiddleNudge => "ZuStack-spacing-mNudge",
        Spacing::Middle => "ZuStack-spacing-m",
        Spacing::Large => "ZuStack-spacing-l",
        Spacing::XLarge => "ZuStack-spacing-xl",
        Spacing::XXLarge => "ZuStack-spacing-xxl",
        Spacing::XXXLarge => "ZuStack-spacing-xxxl",
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub component: String,

    /// Defines the flex-direction style property.
    #[prop_or_default]
    pub direction: Direction,

    /// Add an element between each child.
    #[prop_or_default]
    pub divider: VNode,

    /// Defines the space between immediate children.
    ///
    /// Default is 0.
    #[prop_or_default]
    // TODO(Shaohua): Replace with i32, margin = spacing * 8px.
    pub spacing: Spacing,

    #[prop_or_default]
    pub style: String,

    /// If true, the CSS flexbox gap is used instead of applying margin to children.
    #[prop_or(false)]
    pub use_flex_gap: bool,
}

#[function_component(Stack)]
pub fn stack(props: &Props) -> Html {
    let component = if props.component.is_empty() {
        "div".to_owned()
    } else {
        props.component.clone()
    };

    let cls = classes!(
        "ZuStack-root",
        props.direction.css_class(),
        spacing_cls(props.spacing),
    );

    let style = if props.style.is_empty() {
        None
    } else {
        Some(props.style.clone())
    };

    html! {
        <@{component} class={cls} style={style}>
            {props.children.clone()}
        </@>
    }
}
