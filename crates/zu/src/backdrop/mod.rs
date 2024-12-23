// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// Default value is `div`.
    #[prop_or(AttrValue::from("div"))]
    pub component: AttrValue,

    /// If true, the backdrop is invisible.
    #[prop_or(false)]
    pub invisible: bool,

    /// If true, the component is shown.
    #[prop_or(false)]
    pub open: bool,

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
        },
        props.classes.clone(),
    );

    // TODO(Shaohua): Add transition component.

    html! {
        <@{props.component.to_string()}
            class={root_cls}
            style={props.style.to_attr()}>
            {for props.children.iter()}
        </@>
    }
}
