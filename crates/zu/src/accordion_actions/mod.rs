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

    /// If true, the actions do not have additional margin.
    #[prop_or(false)]
    pub disable_spacing: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(AccordionActions)]
pub fn accordion_actions(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuAccordionActions-root",
        if props.disable_spacing {
            ""
        } else {
            "ZuAccordionActions-spacing"
        },
        props.classes.clone(),
    );

    html! {
        <div class={root_cls} style={props.style.to_attr()}>
            {for props.children.iter()}
        </div>
    }
}
