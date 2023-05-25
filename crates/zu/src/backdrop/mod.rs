// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};
use zu_util::prop;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// If true, the component is shown.
    #[prop_or(false)]
    pub open: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the backdrop is invisible.
    #[prop_or(false)]
    pub invisible: bool,

    #[prop_or_default]
    pub style: AttrValue,

    /// The component used for the transition.
    #[prop_or_default]
    pub transition_component: Option<Html>,

    /// The duration for the transition, in milliseconds.
    #[prop_or_default]
    pub transition_duration: Option<i32>,
}

#[function_component(Backdrop)]
pub fn backdrop(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuBackdrop-root",
        if props.invisible {
            "ZuBackdrop-invisible"
        } else {
            ""
        }
    );

    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };

    // TODO(Shaohua): Add transition component.

    html! {
        <@{component.to_owned()} class={root_cls}
            style={prop::attr_optional(&props.style)}>
            {for props.children.iter()}
        </@>
    }
}
