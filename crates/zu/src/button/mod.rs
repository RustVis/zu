// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod size;
mod variant;

use yew::{
    classes, function_component, html, virtual_dom::VNode, AttrValue, Children, Html, Properties,
};

use crate::styles::color::Color;

// Re-export
pub use size::Size;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, no elevation is used.
    #[prop_or(false)]
    pub disable_elevation: bool,

    /// If true, the keyboard focus ripple is disabled.
    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    /// If true, the ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// Element placed after the children.
    #[prop_or_default]
    pub end_icon: Option<VNode>,

    /// If true, the button will take up the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// The URL to link to when the button is clicked.
    ///
    ///  If defined, an a element will be used as the root node.
    #[prop_or_default]
    pub href: AttrValue,

    /// The size of the component.
    ///
    ///  small is equivalent to the dense button styling.
    #[prop_or_default]
    pub size: Size,

    /// Element placed before the children.
    #[prop_or_default]
    pub start_icon: Option<VNode>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let root_cls = classes!("ZuButton-root");
    html! {
        <button class={root_cls}>
            {for props.children.iter()}
        </button>
    }
}
