// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

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
        }
    );

    let style = if props.style.is_empty() {
        None
    } else {
        Some(props.style.to_string())
    };

    html! {
        <div class={root_cls} style={style}>
            {for props.children.iter()}
        </div>
    }
}