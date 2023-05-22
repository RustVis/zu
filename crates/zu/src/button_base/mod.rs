// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, use_state, AttrValue, Children, Html, Properties};

pub const DEFAULT_LINK_COMPONENT: &str = "a";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    // TODO(Shaohua): Add action ref.
    /// If true, the ripples are centered.
    #[prop_or(false)]
    pub center_ripple: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, the ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// If true, the touch ripple effect is disabled.
    #[prop_or(false)]
    pub disable_touch_ripple: bool,

    /// If true, the base button will have a keyboard focus ripple.
    #[prop_or(false)]
    pub focus_ripple: bool,

    /// This prop can help identify which element has keyboard focus.
    #[prop_or_default]
    pub focus_visible_class: AttrValue,

    /// The component used to render a link when the href prop is provided.
    #[prop_or_default]
    pub link_component: AttrValue,

    // TODO(Shaohua): Add event callback.
    // pub onFocusVisible:
    #[prop_or_default]
    pub style: AttrValue,

    // TODO(Shaohua): Replace with a struct.
    #[prop_or_default]
    pub touch_ripple_props: AttrValue,
    // TODO(Shaohua): Add touchRippleRef
}

#[function_component(ButtonBase)]
pub fn button_base(props: &Props) -> Html {
    let focus_visible = use_state(|| false);

    let root_cls = classes!(
        "ZuButtonBase-root",
        if props.disabled {
            "ZuButtonBase-disabled"
        } else {
            ""
        },
        if *focus_visible {
            "ZuButtonBase-focusVisible"
        } else {
            ""
        },
        if *focus_visible {
            props.focus_visible_class.as_str().to_owned()
        } else {
            String::new()
        }
    );

    // TODO(Shaohua): Handle mouse events.

    let component = if props.component.is_empty() {
        "button"
    } else {
        props.component.as_str()
    };

    html! {
        <@{component.to_owned()} class={root_cls}>
        </@>
    }
}
