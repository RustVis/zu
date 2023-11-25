// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod spacing;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::flex_direction::FlexDirection;
use crate::styles::spacing::Spacing;
use crate::styles::CssClass;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// Default value is `div`.
    #[prop_or(AttrValue::from("div"))]
    pub component: AttrValue,

    /// Defines the flex-direction style property.
    #[prop_or_default]
    pub direction: FlexDirection,

    /// Add an element between each child.
    #[prop_or_default]
    pub divider: Html,

    /// Defines the space between immediate children.
    ///
    /// Default is 0.
    #[prop_or_default]
    // TODO(Shaohua): Replace with i32, margin = spacing * 8px.
    pub spacing: Spacing,

    #[prop_or_default]
    pub style: AttrValue,

    /// If true, the CSS flexbox gap is used instead of applying margin to children.
    #[prop_or(false)]
    pub use_flex_gap: bool,
}

#[function_component(Stack)]
pub fn stack(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuStack-root",
        props.direction.css_class(),
        spacing::css_cls(props.spacing),
        props.classes.clone(),
    );

    // TODO(Shaohua): Add divider elements

    html! {
        <@{props.component.to_string()}
            class={root_cls}
            style={props.style.to_attr()}>
            {for props.children.iter()}
        </@>
    }
}
