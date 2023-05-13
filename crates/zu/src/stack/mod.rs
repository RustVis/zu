// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::direction::Direction;
use crate::styles::spacing::Spacing;
use yew::virtual_dom::VNode;
use yew::{classes, function_component, html, Children, Html, Properties};

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

    let cls = classes!("ZuStack-root");

    html! {
        <@{component} class={cls} style={props.style.clone()}>
        </@>
    }
}
